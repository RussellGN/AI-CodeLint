use crate::genai::invoke_gemini;
use log::{debug, error, trace, warn};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LintResult {
    pub overview: String,
    pub start_line: u32,
    pub end_line: u32,
}

pub async fn lint(text: &str) -> Result<Vec<LintResult>, String> {
    debug!("running lint on document length={}", text.len());
    if text.trim().is_empty() {
        warn!("received empty document text for linting, returning...");
        return Ok(vec![]);
    }

    let preamble = "You are AI CodeLint. Find only real runtime/behavior logic bugs that survive compilation. Ignore style, syntax, type, or IDE/compiler-detectable issues. Return JSON only: [{\"overview\":\"string\",\"start_line\":integer,\"end_line\":integer}]. Use zero-based line numbers. If none, return exactly []. Else return at most 3 items. Each overview: concrete bug + why behavior breaks; no markdown; no speculation.";

    debug!("invoking Gemini linter model");
    let res = invoke_gemini(text, "gemini-2.5-flash-lite", preamble, 500).await?;
    trace!("raw Gemini lint response:\n\n{res}\n");

    let errors_found = serde_json::from_str::<Vec<LintResult>>(&extract_json_array_only(&res)?)
        .map_err(|e| {
            error!("failed to parse lint JSON response: {e}");
            e.to_string()
        })?;

    debug!(
        "lint completed with {} diagnostic{}",
        errors_found.len(),
        if errors_found.len() == 1 { "" } else { "s" }
    );
    Ok(errors_found)
}

fn extract_json_array_only(text: &str) -> Result<&str, String> {
    let open_bracket_index = text.find("[").ok_or("could not find json array")?;
    let end_bracket_index = text.find("]").ok_or("could not find json array")?;
    Ok(&text[open_bracket_index..end_bracket_index + 1])
}
