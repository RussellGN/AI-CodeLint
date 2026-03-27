use crate::genai::invoke_gemini;
use log::debug;

pub struct LintResult {
    pub overview: String,
}

impl LintResult {
    pub fn new(overview: String) -> Self {
        Self { overview }
    }
}

pub async fn lint(text: &str) -> Result<Option<LintResult>, String> {
    let preamble = "You are AI CodeLint, a code logic-bug detector.
    Find only runtime or behavioral logic bugs that can still exist after code compiles.
    Do not report any style, lexical, syntax, nor semantic errors that are caught by a compiler or IDE during development.
    Output rules:
    - If no logic bug is found, respond exactly: No bugs.
    - Otherwise return up to 3 items.
    - Each item must include:
    1) Logic bug
    2) Why it breaks behavior
    Be very concise and concrete, and when uncertain, prefer silence over speculative warnings.";

    debug!("invoking gemini");
    let overview = invoke_gemini(text, "gemini-2.5-flash-lite", preamble, 80).await?;
    debug!("overview received:\n___________\n {overview}\n___________\n");

    if overview.to_lowercase().replace(".", "").trim() == "no bugs" {
        Ok(Some(LintResult::new(overview)))
    } else {
        Ok(None)
    }
}
