use std::collections::HashMap;

use log::{debug, error, trace, warn};
use tokio::sync::Mutex;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use tower_lsp::Client;

use crate::linter::{lint, LintResult};
use crate::lsp::cache;
use crate::DOCS_CACHE_SIZE;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    cached_docs: Mutex<HashMap<String, cache::Document>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cached_docs: Default::default(),
        }
    }

    pub async fn prune_docs_cache(&self) {
        let docs = self.cached_docs.lock().await;
        if docs.len() >= DOCS_CACHE_SIZE {
            // TODO: prune
        }
    }

    pub async fn is_doc_cached(&self, uri: &str) -> bool {
        let docs = self.cached_docs.lock().await;
        docs.contains_key(uri)
    }

    pub async fn cache_doc(&self, uri: &str, doc: cache::Document) {
        let mut docs = self.cached_docs.lock().await;
        docs.insert(uri.to_string(), doc);
    }

    pub async fn replace_doc_text(&self, uri: &str, new_text: String) {
        let mut docs = self.cached_docs.lock().await;
        if let Some(cached_doc) = docs.get_mut(uri) {
            cached_doc.text = new_text;
        }
    }

    pub async fn is_stale(&self, uri: &str, curr_text: &str) -> Result<bool, String> {
        let docs = self.cached_docs.lock().await;
        match docs.get(uri) {
            None => Err(format!("doc not found in cache, uri: {uri}").into()),
            Some(cached_doc) => {
                if cached_doc.hash == cache::Document::hash_text(curr_text) {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
        }
    }

    pub async fn compile_diagnostics(&self, uri: Url) {
        debug!("compiling diagnostics for {}", uri);
        let text = {
            let docs = self.cached_docs.lock().await;
            docs.get(uri.as_str()).map(|doc| doc.text.clone())
        };

        if let Some(text) = text {
            match lint(&text).await {
                Err(e) => {
                    error!("lint failed for {}: {}", uri, e);
                }
                Ok(errs) => {
                    debug!("lint returned {} diagnostics for {}", errs.len(), uri);
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
                        let mut docs = self.cached_docs.lock().await;
                        if let Some(doc) = docs.get_mut(uri.as_str()) {
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
                                uri
                            );
                        }
                    }
                    self.client
                        .publish_diagnostics(uri, diagnostics, None)
                        .await;
                    trace!("published diagnostics");
                }
            }
        } else {
            warn!(
                "cannot compile diagnostics; file not found in cache: {}",
                uri
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
