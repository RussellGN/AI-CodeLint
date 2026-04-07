use semver::Version;
use serde::Deserialize;

pub mod cli;
pub mod inference;
pub mod linter;
pub mod lsp;

pub const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
pub const OPENROUTER_API_KEY: &str = include_str!("../.env");
pub const DOCS_CACHE_SIZE: usize = 20;
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Deserialize)]
struct Status {
    recommended_version: String,
}

pub async fn check_if_outdated() -> Result<(), String> {
    let res = reqwest::get(
        "https://raw.githubusercontent.com/RussellGN/AI-CodeLint/refs/heads/main/status.json",
    )
    .await
    .map_err(|e| e.to_string())?
    .text()
    .await
    .map_err(|e| e.to_string())?;

    let Status {
        recommended_version,
    } = serde_json::from_str(&res).map_err(|e| e.to_string())?;

    let recommended_version = Version::parse(&recommended_version).map_err(|e| e.to_string())?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| e.to_string())?;

    if current_version < recommended_version {
        Err(format!("Current version '{current_version}' of {CRATE_NAME} is out of date. Please download and use the recommended version '{recommended_version}' or newer."))
    } else {
        Ok(())
    }
}
