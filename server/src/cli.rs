use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use tokio::fs;

use crate::linter::lint;

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
}

impl Args {
    pub async fn process(&self) {
        let res = if self.mode != Mode::CLI {
            Err(("cannot run 'process' on args when mode is not set to 'cli'").into())
        } else {
            if let Some(path) = &self.path {
                match fs::try_exists(path).await {
                    Ok(exists) if !exists => Err(format!(
                        "no file found at path: {}",
                        path.display().to_string().yellow().underline()
                    )),
                    Err(e) => Err(format!(
                        "could not traverse path: {}, error: {e}",
                        path.display().to_string().yellow().underline()
                    )),
                    _ => {
                        println!(
                            "{} {}",
                            "running linter on file:".cyan(),
                            path.display().to_string().yellow().underline()
                        );
                        if let Ok(text) = fs::read_to_string(path).await {
                            match lint(&text).await {
                                Err(e) => Err(format!(
                                    "could not lint contents at {}. Error: {e}",
                                    path.display()
                                )),
                                Ok(errors) => {
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
                        } else {
                            Err(format!("could not read contents at {}", path.display()))
                        }
                    }
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
}
