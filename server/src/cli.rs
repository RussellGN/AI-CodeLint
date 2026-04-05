use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use colored::Colorize;
use tokio::fs;

use crate::linter::{lint, LintResult};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Mode {
    Server,
    CLI,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Boost code quality as you type")]
pub struct Args {
    #[arg(short, long)]
    pub mode: Mode,

    #[arg(short, long)]
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(long)]
    pub model: Option<String>,

    #[arg(long)]
    pub max_tokens: Option<u32>,
}

impl Args {
    pub async fn process(&self) {
        let res = if self.mode != Mode::CLI {
            Err(("cannot run 'process' on args when mode is not set to 'cli'").into())
        } else {
            if let Some(path) = &self.path {
                match self.read_file_at_path().await {
                    Ok(text) => {
                        println!(
                            "{} {}",
                            "running linter on file:".cyan(),
                            path.display().to_string().yellow().underline()
                        );
                        match lint(&text, self.model.as_deref(), self.max_tokens).await {
                            Err(e) => Err(format!(
                                "could not lint contents at {}. Error: {e}",
                                path.display()
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

        match res {
            Ok(msg) => {
                println!("{}", msg.cyan());
                std::process::exit(0)
            }
            Err(e) => {
                println!("{}", e.red());
                std::process::exit(1)
            }
        }
    }

    async fn read_file_at_path(&self) -> Result<String, String> {
        let Some(path) = &self.path else {
            return Err("no path provided".into());
        };
        let path_display = path.display().to_string().yellow().underline();
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
        errors.iter().for_each(|lint_err| println!("{lint_err}\n"));
        let err_count = errors.len();
        Ok(format!(
            "found {} bug{} in {}",
            err_count,
            if err_count == 1 { "" } else { "s" },
            path.display()
        ))
    }
}
