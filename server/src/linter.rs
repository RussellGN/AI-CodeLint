use crate::genai::invoke_gemini;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LintResult {
    pub overview: String,
    pub start_line: u32,
    pub end_line: u32,
}

pub async fn lint(text: &str) -> Result<Vec<LintResult>, String> {
    let preamble = "You are AI CodeLint. Find only real runtime/behavior logic bugs that survive compilation. Ignore style, syntax, type, or IDE/compiler-detectable issues. Return JSON only: [{\"overview\":\"string\",\"start_line\":integer,\"end_line\":integer}]. Use zero-based line numbers. If none, return exactly []. Else return at most 3 items. Each overview: concrete bug + why behavior breaks; no markdown; no speculation.";

    debug!("invoking gemini");
    let res = invoke_gemini(text, "gemini-2.5-flash-lite", preamble, 500).await?;
    debug!("overview received:\n___________\n {res}\n___________\n");

    let errors_found = serde_json::from_str::<Vec<LintResult>>(&res).map_err(|e| e.to_string())?;
    Ok(errors_found)
}
