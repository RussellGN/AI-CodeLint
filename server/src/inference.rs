use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ReasoningEffort,
    ResponseFormat, Verbosity,
};
use async_openai::{config::OpenAIConfig, Client};
use log::{debug, error, trace};

use crate::{OPENROUTER_API_KEY, OPENROUTER_BASE_URL};

pub async fn invoke_model(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u32,
    reasoning_effort: ReasoningEffort,
    verbosity: Verbosity,
    response_format: ResponseFormat,
) -> Result<String, String> {
    debug!("invoking model '{model}' | max tokens={max_tokens} | estimate request tokens: prompt={}, preamble={}",
        prompt.estimate_token_count(),
        preamble.estimate_token_count()
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
        .reasoning_effort(reasoning_effort)
        .verbosity(verbosity)
        .response_format(response_format)
        .build()
        .map_err(|e| format!("failed to build inference request: {e}"))?;

    debug!("sending inference request to {model}");
    let res = client.chat().create(req).await.map_err(|e| {
        error!("failed to send inference request: {e}");
        e.to_string()
    })?;

    if let Some(usage) = &res.usage {
        trace!(
            "received {model} response with actual input_tokens={}, and output_tokens={}",
            usage.prompt_tokens,
            usage.completion_tokens
        );
    }

    let res = res.choices.first().expect("inference results are empty!");
    trace!("res:\n\n{res:#?}\n\n");

    res.message
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
