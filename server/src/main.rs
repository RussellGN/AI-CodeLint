mod inference;
mod linter;
mod lsp;

use log::info;
use log::LevelFilter;
use tower_lsp::{LspService, Server};

use crate::lsp::Backend;

const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
const OPENROUTER_API_KEY: &str = include_str!("../.env");
const DOCS_CACHE_SIZE: usize = 20;
const CRATE_NAME: &str = "ai_codelint";

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Off)
        .filter_module(CRATE_NAME, LevelFilter::Trace)
        .init();
    info!("starting {CRATE_NAME} LSP server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend::new(client)).finish();
    info!("LSP service initialized, waiting for editor requests");

    Server::new(stdin, stdout, socket).serve(service).await;
    info!("LSP server stopped");
}
