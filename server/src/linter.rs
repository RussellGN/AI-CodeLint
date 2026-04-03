use std::borrow::Cow;
use std::fmt::Display;

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

impl Display for LintResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line {} to line {}: {}",
            self.start_line, self.end_line, self.overview
        )
    }
}

pub async fn lint(text: &str) -> Result<Vec<LintResult>, String> {
    debug!("running lint on document length={}", text.len());
    if text.trim().is_empty() {
        warn!("received empty document text for linting, returning...");
        return Ok(vec![]);
    }

    let preamble = format!("You are {CRATE_NAME}. Find only real runtime/behavior logic bugs that survive compilation within the provided code. Ignore style, syntax, type, or IDE/compiler-detectable issues. Return JSON only: [{{\"overview\":\"string\",\"start_line\":integer,\"end_line\":integer}}]. Use zero-based line numbers encompassing the entire affected code-block/statements. If none, return exactly []. Else return at most 3 items. Each overview: concrete bug + why behavior breaks; no markdown; no speculation. Do not inlcude whitespace in returned json.");

    // free models
    // let model_id = "qwen/qwen3.6-plus-preview:free";
    let model_id = "nvidia/nemotron-3-super-120b-a12b:free";
    // let model_id = "nvidia/nemotron-3-nano-30b-a3b:free";
    // let model_id = "stepfun/step-3.5-flash:free";
    // let model_id = "arcee-ai/trinity-large-preview:free";

    // paid models - as of april 2
    // let model_id = "xiaomi/mimo-v2-pro"; // #1 programming
    // let model_id = "minimax/minimax-m2.7"; // #2 programming
    // let model_id = "anthropic/claude-opus-4.6"; // #5 programming
    // let model_id = "anthropic/claude-sonnet-4.6"; // #11 programming
    // let model_id = "google/gemini-3-flash-preview"; // #12 programming
    // let model_id = "deepseek/deepseek-v3.2"; // #17 programming

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

    let json = try_to_extract_json(&res)?;
    let errors_found = serde_json::from_str::<Vec<LintResult>>(&json).map_err(|e| {
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

fn try_to_extract_json(text: &str) -> Result<Cow<str>, &str> {
    if let (Some(open_brac_index), Some(close_brac_index)) = (text.find("["), text.find("]")) {
        Ok(Cow::Borrowed(&text[open_brac_index..close_brac_index + 1]))
    } else {
        if let (Some(open_curly_index), Some(close_curly_index)) = (text.find("{"), text.find("}"))
        {
            Ok(Cow::Owned(format!(
                "[{}]",
                &text[open_curly_index..close_curly_index + 1]
            )))
        } else {
            Err("could not find valid json")
        }
    }
}
