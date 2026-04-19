mod backend;

use log::error;
use log::{debug, info, trace, warn};
use tower_lsp::jsonrpc::{self, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

use crate::lsp::backend::Document;
pub use backend::Backend;

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("initializing language server capabilities");

        if self.lsp_startup_errs.len() != 0 {
            let err_msg = format!(
                "ai-codelint encountered errors during lsp server startup: \n{}",
                self.lsp_startup_errs.join(". \n")
            );
            error!("{err_msg}");
            Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::InternalError,
                message: err_msg.into(),
                data: None,
            })
        } else {
            Ok(InitializeResult {
                server_info: None,
                offset_encoding: None,
                capabilities: ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Options(
                        TextDocumentSyncOptions {
                            open_close: Some(true),
                            change: Some(TextDocumentSyncKind::FULL),
                            will_save: Some(false),
                            will_save_wait_until: Some(false),
                            save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                                include_text: Some(true),
                            })),
                        },
                    )),
                    ..Default::default()
                },
            })
        }
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
            self.cache_doc(&uri_str, Document::new(params.text_document.text, vec![]))
                .await;
            debug!("started watching file: {uri_str}");
            self.compile_diagnostics(uri).await;
        };
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        debug!("file changed: {uri}");

        if let Some(changes) = params.content_changes.first() {
            trace!("updating document cache");
            self.replace_doc_text(uri.as_str(), changes.text.clone())
                .await;
            trace!("clearing diagnostics");
            self.client.publish_diagnostics(uri, vec![], None).await;
        } else {
            warn!("'did change' fired with no content changes for: {}", uri);
        };
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        info!("file saved: {uri}");
        let Some(current_text) = params.text else {
            warn!("no text received on 'save', abandoning...");
            return;
        };

        let is_stale = self.is_stale(uri.as_str(), &current_text).await;
        match is_stale {
            Err(e) => warn!("{e}"),
            Ok(false) => trace!("no new changes to recompile diagnostics for: {uri}"),
            Ok(true) => {
                debug!("content has changed since diagnostics were last compiled, recompiling...");
                self.compile_diagnostics(uri).await
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("file closed: {}", params.text_document.uri);
        self.prune_docs_cache().await;
    }
}
