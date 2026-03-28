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
        let mut docs = self.cached_docs.lock().await;
        if docs.len() >= DOCS_CACHE_SIZE {
            trace!("cache is at capacity: {}", docs.len());
            let Some(stale_entry) = docs.iter().min_by_key(|(_, d)| d.diagnostics_version) else {
                return;
            };
            let stale_entry_key = stale_entry.0.clone();
            docs.remove(&stale_entry_key);
            trace!("removed stale doc from cache: {stale_entry_key}");
            trace!("cached docs: {:#?}", docs.keys().collect::<Vec<_>>());
        }
    }

    pub async fn is_doc_cached(&self, uri: &str) -> bool {
        let docs = self.cached_docs.lock().await;
        docs.contains_key(uri)
    }

    pub async fn cache_doc(&self, uri: &str, doc: cache::Document) {
        let mut docs = self.cached_docs.lock().await;
        docs.insert(uri.to_string(), doc);
        trace!("cached docs: {:#?}", docs.keys().collect::<Vec<_>>());
    }

    pub async fn replace_doc_text(&self, uri: &str, new_text: String) {
        let mut docs = self.cached_docs.lock().await;
        if let Some(cached_doc) = docs.get_mut(uri) {
            cached_doc.hash = cache::Document::hash_text(&new_text);
            cached_doc.text = new_text;
        } else {
            warn!("could not find doc {uri} for text replacement")
        }
    }

    pub async fn replace_doc_diags(&self, uri: &str, diags: Vec<Diagnostic>) -> Result<(), String> {
        let mut docs = self.cached_docs.lock().await;
        match docs.get_mut(uri) {
            None => Err(format!("doc not found in cache, uri: {uri}")),
            Some(doc) => {
                doc.diagnostics = diags;
                doc.diagnostics_version += 1;
                Ok(())
            }
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

    fn full_text_range(text: &str) -> Range {
        let lines_count: u32 = text
            .lines()
            .collect::<Vec<_>>()
            .len()
            .try_into()
            .expect("could not narrow usize into u32");
        Range::new(Position::new(0, 0), Position::new(lines_count + 1, 0))
    }

    pub async fn compile_diagnostics(&self, uri: Url) {
        debug!("compiling diagnostics for {}", uri);
        let Some(text_to_compile) = ({
            let docs = self.cached_docs.lock().await;
            docs.get(uri.as_str()).map(|doc| doc.text.clone())
        }) else {
            warn!("cannot compile diagnostics; file not found in cache: {uri}");
            return;
        };

        match lint(&text_to_compile).await {
            Err(e) => error!("lint failed for {uri}: {e}"),
            Ok(errs) => {
                debug!("lint returned {} diagnostics for {uri}", errs.len());
                let full_page_range = Self::full_text_range(&text_to_compile);

                let diagnostics: Vec<Diagnostic> = errs
                    .into_iter()
                    .map(|e| {
                        let mut d: Diagnostic = e.into();
                        d.range = full_page_range; // TODO: remove when confident of AI range output
                        d
                    })
                    .collect();

                if let Err(e) = self
                    .replace_doc_diags(uri.as_str(), diagnostics.clone())
                    .await
                {
                    warn!("{e}")
                }

                self.client
                    .publish_diagnostics(uri, diagnostics, None)
                    .await;
                trace!("published diagnostics");
            }
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
