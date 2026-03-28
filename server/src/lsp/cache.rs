use std::hash::{DefaultHasher, Hash, Hasher};

use tower_lsp::lsp_types::Diagnostic;

#[derive(Debug)]
pub struct Document {
    pub hash: u64,
    pub text: String,
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
            diagnostics,
            text,
        }
    }
}
