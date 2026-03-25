mod genai;
mod linter;
mod lsp;

use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("could not load env vars");
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| lsp::Backend { client }).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
