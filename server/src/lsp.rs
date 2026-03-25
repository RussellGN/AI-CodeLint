use log::debug;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::linter::lint;

#[derive(Debug)]
pub(crate) struct Backend {
    #[allow(unused)]
    pub(crate) client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities::default(),
        })
    }

    async fn shutdown(&self) -> Result<()> {
        debug!("shutdown!");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        debug!("file opened!");
        self.lint_for_errors(CodeDocument {
            text: &params.text_document.text,
            uri: &params.text_document.uri,
        });
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("file changed!");
        let text = match params.content_changes.first() {
            Some(s) => &s.text,
            _ => "",
        };
        self.lint_for_errors(CodeDocument {
            text,
            uri: &params.text_document.uri,
        });
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        debug!("file closed!");
    }
}

impl Backend {
    fn new_diag(text: &str, range: Range) -> Diagnostic {
        Diagnostic {
            message: text.to_string(),
            range: range,
            severity: None,
            code: None,
            code_description: None,
            source: None,
            related_information: None,
            tags: None,
            data: None,
        }
    }

    async fn lint_for_errors(&self, code_to_lint: CodeDocument<'_>) {
        match lint(code_to_lint.text) {
            Err(e) => panic!("{e}"),
            Ok(res) => {
                // TODO: whole document as placeholder for now
                let range = Range::new(
                    Position::new(1, 1),
                    Position::new(
                        code_to_lint.text.len().try_into().unwrap(),
                        code_to_lint.text.len().try_into().unwrap(),
                    ),
                );
                let diagnostics = vec![Self::new_diag(&res.overview, range)];
                self.client
                    .publish_diagnostics(code_to_lint.uri.clone(), diagnostics, None)
                    .await;
            }
        }
    }
}

#[derive(Debug)]
struct CodeDocument<'a> {
    text: &'a str,
    uri: &'a Url,
}
