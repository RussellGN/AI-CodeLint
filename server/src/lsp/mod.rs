use std::collections::HashMap;

use log::{debug, error, info, trace, warn};
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
        info!("initializing language server capabilities");
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
        info!("shutting down language server");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        info!("file opened: {}", uri);
        let mut should_compile = false;
        {
            let mut docs = self.docs_being_watched.lock().await;
            let is_cached = docs.contains_key(&uri.to_string());
            if !is_cached {
                docs.insert(uri.to_string(), CachedDoc::new(uri.to_string(), vec![]));
                should_compile = true;
                debug!("started watching file: {}", uri);
            } else {
                trace!("file already in cache: {}", uri);
            };
        }
        if should_compile {
            self.compile_diagnostics(uri).await
        } else {
            trace!(
                "skipping compile_diagnostics on open for unchanged cache: {}",
                uri
            );
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file changed: {}", uri);
        let Some(changes) = params.content_changes.first() else {
            warn!("did_change without content_changes for {}", uri);
            return;
        };
        let mut should_compile = false;
        {
            let mut cached_docs = self.docs_being_watched.lock().await;
            if let Some(doc) = cached_docs.get_mut(uri.as_str()) {
                if doc.text != changes.text {
                    doc.text = changes.text.clone();
                    should_compile = true;
                    trace!("cached text updated for {}", uri);
                } else {
                    trace!("text unchanged after did_change for {}", uri);
                }
            } else {
                warn!("received change for uncached file: {}", uri);
            }
        }
        if should_compile {
            self.compile_diagnostics(uri).await
        } else {
            trace!(
                "skipping compile_diagnostics because text unchanged: {}",
                uri
            );
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("file closed: {}", params.text_document.uri);
        self.docs_being_watched
            .lock()
            .await
            .remove(params.text_document.uri.as_str());
    }
}

impl Backend {
    async fn compile_diagnostics(&self, doc_uri: Url) {
        debug!("compiling diagnostics for {}", doc_uri);
        let text = {
            let docs = self.docs_being_watched.lock().await;
            docs.get(doc_uri.as_str()).map(|doc| doc.text.clone())
        };

        if let Some(text) = text {
            match lint(&text).await {
                Err(e) => {
                    error!("lint failed for {}: {}", doc_uri, e);
                }
                Ok(errs) => {
                    debug!("lint returned {} diagnostics for {}", errs.len(), doc_uri);
                    let diagnostics: Vec<Diagnostic> = errs.into_iter().map(|e| e.into()).collect();
                    let text_lines_count: u32 = text
                        .lines()
                        .collect::<Vec<_>>()
                        .len()
                        .try_into()
                        .expect("could not narrow usize into u32");
                    let full_page_range =
                        Range::new(Position::new(0, 0), Position::new(text_lines_count + 1, 0));
                    {
                        let mut docs = self.docs_being_watched.lock().await;
                        if let Some(doc) = docs.get_mut(doc_uri.as_str()) {
                            doc.diagnostics = diagnostics
                                .clone()
                                .into_iter()
                                .map(|mut d| {
                                    d.range = full_page_range;
                                    d
                                })
                                .collect();
                        } else {
                            warn!(
                                "file disappeared from cache before diagnostics update: {}",
                                doc_uri
                            );
                        }
                    }
                    self.client
                        .publish_diagnostics(doc_uri, diagnostics, None)
                        .await;
                    trace!("published diagnostics");
                }
            }
        } else {
            warn!(
                "cannot compile diagnostics; file not found in cache: {}",
                doc_uri
            );
        }
    }
}

impl From<LintResult> for Diagnostic {
    fn from(value: LintResult) -> Self {
        // let range = Range::new(
        //     Position::new(value.start_line, 0),
        //     Position {
        //         line: value.end_line + 1,
        //         character: 0,
        //     },
        // );
        let placeholder_range = Range::new(
            Position::new(0, 0),
            Position {
                line: 500,
                character: 0,
            },
        );

        Self {
            range: placeholder_range,
            message: value.overview,
            severity: Some(DiagnosticSeverity::INFORMATION),
            source: Some(String::from("AI CodeLint")),
            tags: None,
            data: None,
            code: None,
            code_description: None,
            related_information: None,
        }
    }
}
