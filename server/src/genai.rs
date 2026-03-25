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
    let client = gemini::Client::new(GEMINI_API_KEY).expect("could not build gemini client");
    let agent = client
        .agent(model)
        .preamble(preamble)
        .max_tokens(max_tokens)
        .build();

    agent.prompt(prompt).await.map_err(|e| e.to_string())
}
