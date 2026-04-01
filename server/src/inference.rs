use async_openai::config::OpenAIConfig;
use async_openai::types::responses::CreateResponseArgs;
use async_openai::Client;
use log::{debug, error, trace};

use crate::{OPENROUTER_API_KEY, OPENROUTER_BASE_URL};

pub async fn invoke_model(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u32,
) -> Result<String, String> {
    debug!("invoking model '{model}' | max tokens={max_tokens} | request lengths: prompt={}, preamble={}",
        prompt.len(),
        preamble.len()
    );

    let config = OpenAIConfig::new()
        .with_api_base(OPENROUTER_BASE_URL)
        .with_api_key(OPENROUTER_API_KEY);

    let client = Client::with_config(config);

    let req = CreateResponseArgs::default()
        .input(prompt)
        .model(model)
        .instructions(preamble)
        .max_output_tokens(max_tokens)
        .build()
        .map_err(|e| e.to_string())?;

    debug!("sending request to {model}");

    let res = client
        .responses()
        .create(req)
        .await
        .map_err(|e| {
            error!("request failed: {e}");
            e.to_string()
        })?
        .output_text()
        .expect("no response text");

    trace!("received {model} response with length={}", res.len());
    Ok(res)
}
