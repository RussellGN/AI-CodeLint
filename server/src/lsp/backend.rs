use std::hash::{DefaultHasher, Hash, Hasher};

use dashmap::DashMap;
use log::{debug, error, trace, warn};
use tokio::time::Instant;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use tower_lsp::Client;

use crate::linter::{lint, LintResult};
use crate::{CRATE_NAME, DOCS_CACHE_SIZE};

#[derive(Debug)]
pub struct Document {
    pub hash: u64,
    pub text: String,
    /// Text hash for __last computed__ diagnostics
    pub diagnostics_hash: Option<u64>,
    pub diagnostics_version: Instant,
    pub diagnostics: Vec<Diagnostic>,
}

impl Document {
    pub fn hash_text(text: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    }

    pub fn new(text: String, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            hash: Self::hash_text(&text),
            diagnostics_version: Instant::now(),
            diagnostics_hash: None,
            diagnostics,
            text,
        }
    }
}

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    cache: DashMap<String, Document>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cache: Default::default(),
        }
    }

    pub async fn prune_docs_cache(&self) {
        if self.cache.len() >= DOCS_CACHE_SIZE {
            trace!("cache is at capacity: {}", self.cache.len());
            let Some(stale_entry) = self.cache.iter().min_by_key(|d| d.diagnostics_version) else {
                return;
            };
            let stale_entry_key = stale_entry.key();
            self.cache.remove(stale_entry_key);
            trace!("removed stale doc from cache: {stale_entry_key}");
            self.print_cache()
        }
    }

    pub async fn is_doc_cached(&self, uri: &str) -> bool {
        self.cache.contains_key(uri)
    }

    fn print_cache(&self) {
        let cache = self
            .cache
            .iter()
            .map(|d| d.key().clone())
            .collect::<Vec<String>>();
        trace!("cached docs: {cache:#?}",);
    }

    pub async fn cache_doc(&self, uri: &str, doc: Document) {
        self.cache.insert(uri.to_string(), doc);
        self.print_cache()
    }

    pub async fn replace_doc_text(&self, uri: &str, new_text: String) {
        if let Some(mut cached_doc) = self.cache.get_mut(uri) {
            cached_doc.hash = Document::hash_text(&new_text);
            cached_doc.text = new_text;
        } else {
            warn!("could not find doc {uri} for text replacement")
        }
    }

    pub async fn replace_doc_diags(&self, uri: &str, diags: Vec<Diagnostic>) -> Result<(), String> {
        match self.cache.get_mut(uri) {
            None => Err(format!("doc not found in cache, uri: {uri}")),
            Some(mut doc) => {
                doc.diagnostics = diags;
                doc.diagnostics_version = Instant::now();
                doc.diagnostics_hash = Some(doc.hash);
                Ok(())
            }
        }
    }

    pub async fn is_stale(&self, uri: &str, curr_text: &str) -> Result<bool, String> {
        match self.cache.get(uri) {
            None => Err(format!("doc not found in cache, uri: {uri}").into()),
            Some(cached_doc) => {
                if cached_doc
                    .diagnostics_hash
                    .is_some_and(|hash| hash == Document::hash_text(curr_text))
                {
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
            .unwrap_or_default();
        Range::new(Position::new(0, 0), Position::new(lines_count + 1, 0))
    }

    pub async fn compile_diagnostics(&self, uri: Url) {
        debug!("compiling diagnostics for {}", uri);
        let Some(text_to_compile) = ({ self.cache.get(uri.as_str()).map(|doc| doc.text.clone()) })
        else {
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

                trace!("{diagnostics:#?}");

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
        let range = Range::new(
            Position::new(value.start_line, 0),
            Position {
                line: value.end_line + 1,
                character: 0,
            },
        );
        // let placeholder_range = Range::new(
        //     Position::new(0, 0),
        //     Position {
        //         line: 500,
        //         character: 0,
        //     },
        // );

        Self {
            range,
            message: value.overview,
            severity: Some(DiagnosticSeverity::INFORMATION),
            source: Some(String::from(CRATE_NAME)),
            tags: None,
            data: None,
            code: None,
            code_description: None,
            related_information: None,
        }
    }
}
