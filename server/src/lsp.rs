use std::collections::HashMap;

use log::debug;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::linter::{lint, LintResult};

#[derive(Debug)]
pub struct CachedDoc {
    text: String,
    // hash: String,
    diagnostics: Vec<Diagnostic>,
}

impl CachedDoc {
    pub fn new(text: String, diagnostics: Vec<Diagnostic>) -> Self {
        Self { text, diagnostics }
    }
}

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub docs_being_watched: Mutex<HashMap<String, CachedDoc>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        debug!("initializing server");
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn shutdown(&self) -> Result<()> {
        debug!("shutting down server");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file opened: {}", uri);
        let mut should_compile = false;
        {
            let mut docs = self.docs_being_watched.lock().await;
            let is_cached = docs.contains_key(&uri.to_string());
            if !is_cached {
                docs.insert(uri.to_string(), CachedDoc::new(uri.to_string(), vec![]));
                should_compile = true;
            };
        }
        if should_compile {
            self.compile_diagnostics(uri).await
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file changed: {}", uri);
        let Some(changes) = params.content_changes.first() else {
            return;
        };
        let mut should_compile = false;
        {
            let mut cached_docs = self.docs_being_watched.lock().await;
            if let Some(doc) = cached_docs.get_mut(uri.as_str()) {
                if doc.text != changes.text {
                    doc.text = changes.text.clone();
                    should_compile = true;
                }
            }
        }
        if should_compile {
            self.compile_diagnostics(uri).await
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        debug!("file closed: {}", params.text_document.uri);
        self.docs_being_watched
            .lock()
            .await
            .remove(params.text_document.uri.as_str());
    }
}

impl Backend {
    async fn compile_diagnostics(&self, doc_uri: Url) {
        let text = {
            let docs = self.docs_being_watched.lock().await;
            docs.get(doc_uri.as_str()).map(|doc| doc.text.clone())
        };

        if let Some(text) = text {
            match lint(&text).await {
                Err(e) => panic!("{e}"),
                Ok(errs) => {
                    let diagnostics: Vec<Diagnostic> = errs.into_iter().map(|e| e.into()).collect();
                    {
                        let mut docs = self.docs_being_watched.lock().await;
                        if let Some(doc) = docs.get_mut(doc_uri.as_str()) {
                            doc.diagnostics = diagnostics.clone();
                        }
                    }
                    self.client
                        .publish_diagnostics(doc_uri, diagnostics, None)
                        .await;
                }
            }
        }
    }
}

impl From<LintResult> for Diagnostic {
    fn from(value: LintResult) -> Self {
        Self {
            range: Range::new(
                Position::new(value.start_line, 10000),
                Position {
                    line: value.end_line,
                    character: 10_000,
                },
            ),
            message: value.overview,
            severity: Some(DiagnosticSeverity::HINT),
            source: Some(String::from("AI CodeLint")),
            tags: None,
            data: None,
            code: None,
            code_description: None,
            related_information: None,
        }
    }
}
