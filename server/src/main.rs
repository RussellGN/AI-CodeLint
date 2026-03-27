mod genai;
mod linter;
mod lsp;

use log::info;
use log::LevelFilter;
use tower_lsp::{LspService, Server};

const GEMINI_API_KEY: &'static str = include_str!("../.env");

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Off)
        .filter_module("ai_codelint", LevelFilter::Trace)
        .init();
    info!("starting AI CodeLint LSP server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| lsp::Backend {
        client,
        docs_being_watched: Default::default(),
    })
    .finish();

    info!("LSP service initialized, waiting for editor requests");

    Server::new(stdin, stdout, socket).serve(service).await;
    info!("LSP server stopped");
}
