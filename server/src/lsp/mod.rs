mod backend;
mod cache;

use log::{debug, info, trace, warn};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

pub use backend::Backend;

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
        let uri_str = uri.to_string();
        info!("file opened: {uri}");

        if self.is_doc_cached(&uri_str).await {
            trace!("skipping compile_diagnostics on open for unchanged cached doc: {uri_str}",);
        } else {
            self.cache_doc(&uri_str, cache::Document::new(uri_str.to_string(), vec![]))
                .await;
            debug!("started watching file: {uri_str}");
            self.compile_diagnostics(uri).await;
        };
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file changed: {}", uri);

        if let Some(changes) = params.content_changes.first() {
            self.replace_doc_text(uri.as_str(), changes.text.clone())
                .await;
        } else {
            warn!("did_change fired with no content changes for {}", uri);
        };
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file saved: {}", uri);
        let Some(current_text) = params.text else {
            warn!("no text received on 'save', abandoning...");
            return;
        };

        let is_stale = self.is_stale(uri.as_str(), &current_text).await;
        match is_stale {
            Ok(true) => {
                debug!("content has changed since diagnostics were last compiled, recompiling...");
                self.compile_diagnostics(uri).await
            }
            Err(e) => warn!("{e}"),
            _ => trace!("no new changes to recompile diagnostics for: {uri}"),
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("file closed: {}", params.text_document.uri);
        self.prune_docs_cache().await;
    }
}
