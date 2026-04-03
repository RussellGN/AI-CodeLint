use clap::{Parser, ValueEnum};
use log::debug;

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
}

impl Args {
    pub fn process(&self) {
        debug!("cli:\n{self:#?}");
    }
}
