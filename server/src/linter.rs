use async_openai::types::chat::{ResponseFormat, Verbosity};
use log::{debug, error, trace, warn};
use serde::Deserialize;

use crate::inference::invoke_model;
use crate::CRATE_NAME;

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

    let preamble = format!("You are {CRATE_NAME}. Find only real runtime/behavior logic bugs that survive compilation within the provided code. Ignore style, syntax, type, or IDE/compiler-detectable issues. Return JSON only: [{{\"overview\":\"string\",\"start_line\":integer,\"end_line\":integer}}]. Use zero-based line numbers encompassing the entire affected code-block/statements. If none, return exactly []. Else return at most 3 items. Each overview: concrete bug + why behavior breaks; no markdown; no speculation. Do not inlcude whitespace in returned json.");

    let model_id = "qwen/qwen3.6-plus-preview:free";
    let res = invoke_model(
        text,
        model_id,
        &preamble,
        500,
        Verbosity::Medium,
        ResponseFormat::JsonObject,
    )
    .await?;
    trace!("raw lint response:\n\n{res}\n");

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
