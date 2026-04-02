use async_openai::types::responses::{CreateResponseArgs, ReasoningArgs};
use async_openai::Client;
use async_openai::{config::OpenAIConfig, types::responses::Reasoning};
use log::{debug, error, trace};

use crate::{OPENROUTER_API_KEY, OPENROUTER_BASE_URL};

pub async fn invoke_model(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u32,
) -> Result<String, String> {
    debug!("invoking model '{model}' | max tokens={max_tokens} | estimate request tokens: prompt={}, preamble={}",
        prompt.estimate_token_count(),
        preamble.estimate_token_count()
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

    let res = client.responses().create(req).await.map_err(|e| {
        error!("request failed: {e}");
        e.to_string()
    })?;

    if let Some(usage) = &res.usage {
        trace!(
            "received {model} response with actual input_tokens={}, and output tokens={}",
            usage.input_tokens,
            usage.output_tokens
        );
    }

    Ok(res.output_text().expect("no response text"))
}

trait TokenCount {
    fn estimate_token_count(&self) -> usize;
}

impl<T: AsRef<str>> TokenCount for T {
    fn estimate_token_count(&self) -> usize {
        (self.as_ref().len() / 3).max(1)
    }
}
