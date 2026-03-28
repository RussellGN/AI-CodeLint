mod genai;
mod linter;
mod lsp;

use log::info;
use log::LevelFilter;
use tower_lsp::{LspService, Server};

use crate::lsp::Backend;

const GEMINI_API_KEY: &'static str = include_str!("../.env");
const DOCS_CACHE_SIZE: usize = 20;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Off)
        .filter_module("ai_codelint", LevelFilter::Trace)
        .init();
    info!("starting AI CodeLint LSP server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend::new(client)).finish();
    info!("LSP service initialized, waiting for editor requests");

    Server::new(stdin, stdout, socket).serve(service).await;
    info!("LSP server stopped");
}
