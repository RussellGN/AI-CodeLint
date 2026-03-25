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

    async fn did_open(&self, _params: DidOpenTextDocumentParams) {
        debug!("file opened!");
    }

    async fn did_change(&self, _params: DidChangeTextDocumentParams) {
        debug!("file changed!");
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        debug!("file saved!");
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        debug!("file closed!");
    }
}
