use async_openai::error::OpenAIError;
use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ResponseFormat,
    Verbosity,
};
use async_openai::{config::OpenAIConfig, Client};
use log::{debug, error};
use serde_json::Value;

use crate::{ OPENROUTER_BASE_URL, get_api_key, warn_if_free_model};

pub async fn invoke_model(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u32,
    verbosity: Verbosity,
    response_format: ResponseFormat,
) -> Result<String, String> {
    debug!(
        "invoke {model}, with {} estimated input tokens, and max output tokens preference of {max_tokens}",
        prompt.estimate_token_count() + preamble.estimate_token_count()
    );
    warn_if_free_model(model, Some(true));

    let config = OpenAIConfig::new()
        .with_api_base(OPENROUTER_BASE_URL)
        .with_api_key(get_api_key());
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
        error!("inference request failed: {e}");
        let err_message = match e {
            OpenAIError::ApiError(e) => format!("{model} rejected the request: {}",  e.message),
            OpenAIError::FileSaveError(e) => format!("Failed to save a file needed for the request: {e}"),
            OpenAIError::FileReadError(e) => format!("Failed to read a file needed for the request: {e}"),
            OpenAIError::Reqwest(_) => format!("Could not reach {model} - check your internet connection and try again"),
            OpenAIError::StreamError(_) => format!("The connection to {model} was interrupted mid-response. Try again."),
            OpenAIError::InvalidArgument(e) => 
                format!("An invalid argument was passed to {model}. This is likely a bug - please report it. ({e})",),
            OpenAIError::JSONDeserialize(_, raw) => {
                // try to pull a human-readable message out of the raw error body
                let hint = serde_json::from_str::<Value>(&raw)
                    .ok()
                    .and_then(|v| v["error"]["message"].as_str().map(str::to_owned))
                    .unwrap_or( raw);
                format!("OpenRouter returned an error: {hint}")
            }

        };
        err_message
    })?;

    if let Some(usage) = &res.usage {
        debug!(
            "{model} processed {} input tokens, and produced {} output tokens",
            usage.prompt_tokens, usage.completion_tokens
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
