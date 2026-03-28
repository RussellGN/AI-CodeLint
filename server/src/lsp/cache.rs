use tower_lsp::lsp_types::Diagnostic;

#[derive(Debug)]
pub struct CachedDoc {
    pub text: String,
    // hash: String,
    pub diagnostics: Vec<Diagnostic>,
}

impl CachedDoc {
    pub fn new(text: String, diagnostics: Vec<Diagnostic>) -> Self {
        Self { text, diagnostics }
    }
}
