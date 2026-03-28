use std::collections::HashMap;

use log::{debug, error, trace, warn};
use tokio::sync::Mutex;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use tower_lsp::Client;

use crate::{
    linter::{lint, LintResult},
    lsp::cache::CachedDoc,
};

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub docs_being_watched: Mutex<HashMap<String, CachedDoc>>,
}

impl Backend {
    pub async fn compile_diagnostics(&self, doc_uri: Url) {
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
