pub mod cli;
pub mod config;
pub mod inference;
pub mod linter;
pub mod lsp;

use std::path::Path;

use colored::{ColoredString, Colorize};
use semver::Version;
use serde::Deserialize;

pub const OPENROUTER_API_KEY_VARNAME: &'static str = "OPENROUTER_API_KEY";
pub const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
pub const OPENROUTER_API_KEY_DASH_URL: &str = "https://openrouter.ai/keys";
pub const DOCS_CACHE_SIZE: usize = 20;
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Deserialize)]
struct Status {
    recommended_version: String,
}

pub async fn check_if_outdated() -> Result<(), String> {
    let res = reqwest::get(
        "https://raw.githubusercontent.com/RussellGN/AI-CodeLint/refs/heads/main/status.json",
    )
    .await
    .map_err(|e| e.to_string())?
    .text()
    .await
    .map_err(|e| e.to_string())?;

    let Status {
        recommended_version,
    } = serde_json::from_str(&res).map_err(|e| e.to_string())?;

    let recommended_version = Version::parse(&recommended_version).map_err(|e| e.to_string())?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| e.to_string())?;

    if current_version < recommended_version {
        Err(format!("Current version '{current_version}' of {CRATE_NAME} is out of date. Please download and use the recommended version '{recommended_version}' or newer."))
    } else {
        Ok(())
    }
}

pub trait CLIFormatter {
    fn success_display(self) -> ColoredString;
    fn info_display(self) -> ColoredString;
    fn warning_display(self) -> ColoredString;
    fn error_display(self) -> ColoredString;
    fn path_display(self) -> ColoredString;
    fn default_display(self) -> ColoredString;
    fn prompt_display(self) -> ColoredString;
    fn normal_display(self) -> ColoredString;
}

impl<T: Colorize> CLIFormatter for T {
    fn success_display(self) -> ColoredString {
        self.bold().bright_green()
    }

    fn warning_display(self) -> ColoredString {
        // self.bold().color(colored::Color::TrueColor {
        //     r: 240,
        //     g: 158,
        //     b: 108,
        // })
        format!("{} {}", "!!".bold().red(), self.bold()).into()
    }

    fn info_display(self) -> ColoredString {
        self.bold().cyan()
    }

    fn error_display(self) -> ColoredString {
        self.bold().red()
    }

    fn path_display(self) -> ColoredString {
        // self.bold().underline().yellow()
        self.bold().underline()
    }

    fn default_display(self) -> ColoredString {
        format!(
            "[{}]",
            self.color(colored::Color::TrueColor {
                r: 160,
                g: 160,
                b: 160,
            })
        )
        .into()
    }

    fn prompt_display(self) -> ColoredString {
        self.bold()
    }

    fn normal_display(self) -> ColoredString {
        self.bold()
    }
}

pub fn get_api_key() -> String {
    match std::env::var(OPENROUTER_API_KEY_VARNAME) {
        Ok(key) => key,
        Err(e) => {
            println!(
                "{}: {e}",
                "'OPENROUTER_API_KEY' environment variable is required".error_display()
            );
            std::process::exit(1)
        }
    }
}

pub fn get_api_key_fallable() -> Result<String, String> {
    std::env::var(OPENROUTER_API_KEY_VARNAME).map_err(|e| e.to_string())
}

pub fn get_file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.display().to_string())
        .unwrap_or_default()
}
