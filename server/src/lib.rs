pub mod cli;
pub mod inference;
pub mod linter;
pub mod lsp;

pub const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
pub const OPENROUTER_API_KEY: &str = include_str!("../.env");
pub const DOCS_CACHE_SIZE: usize = 20;
pub const CRATE_NAME: &str = "ai_codelint";
