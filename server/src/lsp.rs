use log::debug;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

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
        });
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("file changed!");
        let text = match params.content_changes.first() {
            Some(s) => &s.text,
            _ => "",
        };
        self.lint_for_errors(CodeDocument { text });
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        debug!("file closed!");
    }
}

impl Backend {
    fn lint_for_errors(&self, code_to_lint: CodeDocument) {
        debug!("linting_____________________________________________________________");
        debug!("\n\n{}\n\n", code_to_lint.text);
        debug!("done linting________________________________________________________");
    }
}

#[derive(Debug)]
struct CodeDocument<'a> {
    text: &'a str,
}
