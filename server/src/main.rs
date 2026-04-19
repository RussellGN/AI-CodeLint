use clap::Parser;
use log::{info, LevelFilter};
use tower_lsp::{LspService, Server};

use ai_codelint::cli::{Args, Mode};
use ai_codelint::config::Config;
use ai_codelint::lsp::Backend;
use ai_codelint::{check_if_outdated, CLIFormatter, CRATE_NAME};

#[tokio::main]
async fn main() {
    let mut lsp_startup_errs = vec![];
    dotenvy::dotenv().ok();
    let args = Args::parse();

    if let Err(e) = check_if_outdated().await {
        println!("{} - {e}", "version check failed".error_display());
        if args.mode == Some(Mode::Server) {
            lsp_startup_errs.push(format!("{} - {e}", "version check failed".error_display()));
        } else {
            std::process::exit(1)
        }
    }

    if args.configure {
        let config_result = match Config::build().await {
            Ok(mut config) => config.walkthrough().await,
            Err(e) => Err(e),
        };
        if let Err(e) = config_result {
            println!("{}: {e}", "configuration failed".error_display());
            std::process::exit(1)
        }
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

        let (service, socket) =
            LspService::build(|client| Backend::new(client, lsp_startup_errs)).finish();
        info!("LSP service initialized, waiting for editor requests");

        Server::new(stdin, stdout, socket).serve(service).await;
        info!("LSP server stopped");
    }

    if args.mode.is_none() && !args.configure {
        println!(
            "{}",
            "no args provided, run with --help for documentation".error_display()
        );
        std::process::exit(1)
    }
}
