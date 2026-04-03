use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use log::error;
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
    pub verbose: Option<bool>,
}

impl Args {
    pub async fn process(&self) {
        if self.mode != Mode::CLI {
            error!("cannot run 'process' on args when mode is not set to 'cli'");
            return;
        }

        let res = if let Some(path) = &self.path {
            println!("running linter on file: {}", path.display());
            if let Ok(text) = fs::read_to_string(path).await {
                match lint(&text).await {
                    Err(e) => Err(format!(
                        "could not lint contents at {}. Error: {e}",
                        path.display()
                    )),
                    Ok(errors) => {
                        errors.iter().for_each(|lint_err| println!("{lint_err}\n"));
                        Ok(format!("found {} bugs in {}", errors.len(), path.display()))
                    }
                }
            } else {
                Err(format!("could not read contents at {}", path.display()))
            }
        } else {
            Err("'path' argument is required. Run with --help for usage instructions".into())
        };

        match res {
            Ok(msg) => {
                println!("{msg}");
                std::process::exit(0)
            }
            Err(e) => {
                println!("{e}");
                std::process::exit(1)
            }
        }
    }
}
