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
        info!("file opened: {}", uri);
        let mut should_compile = false;
        {
            let mut docs = self.cached_docs.lock().await;
            let is_cached = docs.contains_key(&uri.to_string());
            if !is_cached {
                docs.insert(
                    uri.to_string(),
                    cache::Document::new(uri.to_string(), vec![]),
                );
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

        if let Some(changes) = params.content_changes.first() {
            let mut docs = self.cached_docs.lock().await;
            let Some(cached_doc) = docs.get_mut(uri.as_str()) else {
                return;
            };
            cached_doc.text = changes.text.clone();
        } else {
            warn!("did_change without content_changes for {}", uri);
        };
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file saved: {}", uri);
        let Some(saved_text) = params.text else {
            warn!("no text received on 'save', abandoning...");
            return;
        };

        let should_recompile = {
            let docs = self.cached_docs.lock().await;
            let Some(cached_doc) = docs.get(uri.as_str()) else {
                return;
            };
            cached_doc.text == saved_text
        };

        if should_recompile {
            debug!("content has changed since diagnostics were last compiled, recompiling...");
            self.compile_diagnostics(uri).await
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("file closed: {}", params.text_document.uri);
        self.cached_docs
            .lock()
            .await
            .remove(params.text_document.uri.as_str());
    }
}
