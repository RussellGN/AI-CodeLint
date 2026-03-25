use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::gemini;

pub async fn invoke_gemini(
    prompt: &str,
    model: &str,
    preamble: &str,
    max_tokens: u64,
) -> Result<String, String> {
    let client = gemini::Client::from_env();
    let agent = client
        .agent(model)
        .preamble(preamble)
        .max_tokens(max_tokens)
        .build();

    agent.prompt(prompt).await.map_err(|e| e.to_string())
}
