use ai_codelint::config::Config;
use ai_codelint::{check_if_outdated, CRATE_NAME};
use clap::Parser;
use colored::Colorize;
use log::{info, LevelFilter};
use tower_lsp::{LspService, Server};

use ai_codelint::cli::{Args, Mode};
use ai_codelint::lsp::Backend;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(e) = check_if_outdated().await {
        println!("{}: {e}", "version check failed".bold().red());
        std::process::exit(1)
    }

    let args = Args::parse();

    if args.configure {
        Config::walkthrough();
        return;
    }

    if args.mode == Some(Mode::Server) || args.verbose {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Off)
            .filter_module(
                &CRATE_NAME.to_string().replace("-", "_"),
                LevelFilter::Trace,
            )
            .init();
    }

    if args.mode == Some(Mode::CLI) {
        let _ = clearscreen::clear();
        info!("running in CLI mode!");
        args.process().await;
    } else if args.mode == Some(Mode::Server) {
        info!("starting {CRATE_NAME} LSP server");

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, socket) = LspService::build(|client| Backend::new(client)).finish();
        info!("LSP service initialized, waiting for editor requests");

        Server::new(stdin, stdout, socket).serve(service).await;
        info!("LSP server stopped");
    }

    if args.mode.is_none() && !args.configure {
        println!(
            "{}",
            "no args provided, run with --help for documentation"
                .bold()
                .red()
        );
        std::process::exit(1)
    }
}
