use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ResponseFormat,
    Verbosity,
};
use async_openai::{config::OpenAIConfig, Client};
use log::{debug, error, trace};

use crate::{OPENROUTER_API_KEY, OPENROUTER_BASE_URL};

pub async fn invoke_model(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u32,
    verbosity: Verbosity,
    response_format: ResponseFormat,
) -> Result<String, String> {
    debug!(
        "invoking model '{model}' | max tokens={max_tokens} | estimate input tokens={}",
        prompt.estimate_token_count() + preamble.estimate_token_count()
    );

    let config = OpenAIConfig::new()
        .with_api_base(OPENROUTER_BASE_URL)
        .with_api_key(OPENROUTER_API_KEY);
    let client = Client::with_config(config);

    let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(preamble)
            .build()
            .map_err(|e| format!("failed to build inference system message: {e}"))?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()
            .map_err(|e| format!("failed to build inference lint prompt: {e}"))?
            .into(),
    ];

    let req = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .max_completion_tokens(max_tokens)
        .n(1)
        .verbosity(verbosity)
        .response_format(response_format)
        .build()
        .map_err(|e| format!("failed to build inference request: {e}"))?;

    debug!("sending inference request to {model}");
    let res = client.chat().create(req).await.map_err(|e| {
        error!("inference request failed:\n\n {e:#?}\n");
        e.to_string()
    })?;

    if let Some(usage) = &res.usage {
        trace!(
            "received {model} response with actual input_tokens={}, and output_tokens={}",
            usage.prompt_tokens,
            usage.completion_tokens
        );
    }

    let choice = res
        .choices
        .first()
        .ok_or_else(|| "inference results are empty!")?;

    choice
        .message
        .content
        .clone()
        .ok_or(String::from("no response text"))
}

trait TokenCount {
    fn estimate_token_count(&self) -> usize;
}

impl<T: AsRef<str>> TokenCount for T {
    fn estimate_token_count(&self) -> usize {
        (self.as_ref().len() / 3).max(1)
    }
}
