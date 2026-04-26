use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use tokio::fs;

use crate::{
    linter::{lint, LintResult},
    CLIFormatter,
};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Mode {
    #[value(help = "Run as an LSP server for editor integrations.")]
    Server,
    #[value(help = "Run once from the terminal against a local file path.")]
    CLI,
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Use LLMs to lint code.",
    long_about = "AI-CodeLint uses LLMs (through inference providers) to analyze your source code and identify logic bugs.\n\nRun in server mode to power editor diagnostics over LSP, or run in cli mode to lint one file from the terminal.",
    after_long_help = "Examples:\n  ai-codelint --mode server\n  ai-codelint --mode cli --path ./src/app.ts\n  ai-codelint --mode cli --path ./src/app.ts --model anthropic/claude-sonnet-4 --max-tokens 4096\n  ai-codelint --configure\n\nNotes:\n  - --path is required when --mode cli is used.\n  - --model and --max-tokens apply to linting requests.\n  - Set OPENROUTER_API_KEY in your environment before linting.",
    next_line_help = true
)]
pub struct Args {
    #[arg(
        short,
        long,
        value_enum,
        value_name = "MODE",
        help = "Execution mode: server for LSP, cli for one-shot file linting."
    )]
    pub mode: Option<Mode>,

    #[arg(
        short,
        long,
        value_name = "FILE_PATH",
        required_if_eq("mode", "cli"),
        requires = "mode",
        help = "Path to the source file to lint. Required when --mode cli is used."
    )]
    pub path: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Enable verbose logs (also enabled automatically in server mode)."
    )]
    pub verbose: bool,

    #[arg(
        short,
        long,
        help = "Run interactive configuration setup and write/update local config."
    )]
    pub configure: bool,

    #[arg(
        long,
        value_name = "MODEL",
        requires = "mode",
        help = "Model identifier used for linting requests (for example anthropic/claude-sonnet-4)."
    )]
    pub model: Option<String>,

    #[arg(
        long,
        value_name = "TOKENS",
        requires = "mode",
        help = "Maximum response token budget for linting output."
    )]
    pub max_tokens: Option<u32>,
}

impl Args {
    pub async fn process(&self) {
        let res = if self.mode != Some(Mode::CLI) {
            Err(("cannot run 'process' on args when mode is not set to 'cli'").into())
        } else {
            if let Some(path) = &self.path {
                match self.read_file_at_path().await {
                    Ok(text) => {
                        println!(
                            "{} {}",
                            "running linter on file:".info_display(),
                            path.display().to_string().path_display()
                        );
                        let filename = self
                            .path
                            .clone()
                            .unwrap_or_default()
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        match lint(
                            false,
                            &filename,
                            &text,
                            self.model.as_deref(),
                            self.max_tokens,
                        )
                        .await
                        {
                            Err(e) => Err(format!(
                                "could not lint contents at {}\n--> {e}",
                                path.display().to_string().path_display()
                            )),
                            Ok(errors) => Self::print_lint_errors(&errors, path),
                        }
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err("'path' argument is required. Run with --help for usage instructions".into())
            }
        };

        println!();

        match res {
            Ok(msg) => {
                println!("{}", msg.success_display());
                std::process::exit(0)
            }
            Err(e) => {
                eprintln!("{}", e.error_display());
                std::process::exit(1)
            }
        }
    }

    async fn read_file_at_path(&self) -> Result<String, String> {
        let Some(path) = &self.path else {
            return Err("no path provided".into());
        };
        let path_display = path.display().to_string().path_display();
        match fs::try_exists(path).await {
            Ok(exists) if !exists => Err(format!("no file found at path: {path_display}")),
            Err(e) => Err(format!("could not traverse: {path_display}, error: {e}",)),
            _ => fs::read_to_string(path)
                .await
                .map_err(|e| format!("could not read contents at {}, error: {e}", path.display())),
        }
    }

    fn print_lint_errors(errors: &Vec<LintResult>, path: &Path) -> Result<String, String> {
        if errors.len() != 0 {
            println!("");
        }
        errors
            .iter()
            .for_each(|lint_err| println!("{}", format!("{lint_err}\n").error_display()));
        let err_count = errors.len();
        Ok(format!(
            "found {} logic bug{} in {}",
            err_count,
            if err_count == 1 { "" } else { "s" },
            path.display()
        ))
    }
}
