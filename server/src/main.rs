mod genai;
mod linter;
mod lsp;

use tower_lsp::{LspService, Server};

const GEMINI_API_KEY: &'static str = include_str!("../../.env");

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| lsp::Backend { client }).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
