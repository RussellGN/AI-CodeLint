mod cli;
mod inference;
mod linter;
mod lsp;

use clap::Parser;
use log::{info, LevelFilter};
use tower_lsp::{LspService, Server};

use crate::cli::Mode;
use crate::lsp::Backend;

const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
const OPENROUTER_API_KEY: &str = include_str!("../.env");
const DOCS_CACHE_SIZE: usize = 20;
const CRATE_NAME: &str = "ai_codelint";

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    if args.mode == Mode::Server || args.verbose {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Off)
            .filter_module(CRATE_NAME, LevelFilter::Trace)
            .init();
    }

    if args.mode == Mode::CLI {
        let _ = clearscreen::clear();
        info!("running in CLI mode!");
        args.process().await;
    } else {
        info!("starting {CRATE_NAME} LSP server");

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, socket) = LspService::build(|client| Backend::new(client)).finish();
        info!("LSP service initialized, waiting for editor requests");

        Server::new(stdin, stdout, socket).serve(service).await;
        info!("LSP server stopped");
    }
}
