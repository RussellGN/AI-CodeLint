use log::{debug, error, trace};
use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::gemini;

use crate::GEMINI_API_KEY;

pub async fn invoke_gemini(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u64,
) -> Result<String, String> {
    debug!("preparing Gemini client for model={model} max_tokens={max_tokens}");
    trace!(
        "gemini request sizes: prompt_bytes={} preamble_bytes={}",
        prompt.len(),
        preamble.len()
    );

    let client = gemini::Client::new(GEMINI_API_KEY).map_err(|e| {
        error!("failed to build Gemini client: {e}");
        e.to_string()
    })?;

    let agent = client
        .agent(model)
        .preamble(preamble)
        .max_tokens(max_tokens)
        .build();

    debug!("sending prompt to Gemini");

    let response = agent.prompt(prompt).await.map_err(|e| {
        error!("Gemini prompt failed: {e}");
        e.to_string()
    })?;

    trace!("received Gemini response bytes={}", response.len());
    Ok(response)
}
