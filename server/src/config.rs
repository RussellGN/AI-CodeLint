use std::{io::Write, path::PathBuf, thread};

use colored::Colorize;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{
    get_api_key_fallable, CLIFormatter, CRATE_NAME, OPENROUTER_API_KEY_DASH_URL,
    OPENROUTER_API_KEY_VARNAME,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: Option<PathBuf>,
    model: String,
    max_tokens: u32,
}

pub const RECOMMENDED_MODEL: &str = "anthropic/claude-sonnet-4.6";
const DEFAULT_MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";
const DEFAULT_MAX_TOKEN_USAGE: u32 = 500;

impl Default for Config {
    fn default() -> Self {
        Config {
            path: None,
            max_tokens: DEFAULT_MAX_TOKEN_USAGE,
            model: DEFAULT_MODEL.to_owned(),
        }
    }
}

impl Config {
    pub async fn build() -> Result<Self, String> {
        let project = ProjectDirs::from("com", "russellgn", CRATE_NAME)
            .ok_or("could not determine config directory")?;

        let config_dir = project.config_dir();
        let config_path = config_dir.join("config.json");
        let config_exists = tokio::fs::try_exists(&config_path)
            .await
            .map_err(|e| e.to_string())?;

        if config_exists {
            let content = tokio::fs::read_to_string(config_path)
                .await
                .map_err(|e| e.to_string())?;
            let config = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(config)
        } else {
            tokio::fs::create_dir_all(config_dir)
                .await
                .map_err(|e| e.to_string())?;
            let config = Config {
                path: Some(config_path.clone()),
                ..Config::default()
            };
            let contents = serde_json::to_string(&config).map_err(|e| e.to_string())?;
            tokio::fs::write(config_path, contents)
                .await
                .map_err(|e| e.to_string())?;
            Ok(config)
        }
    }

    fn prompt_user_and(prompt: String) -> Result<String, String> {
        let thread_handle = thread::spawn(move || {
            print!("{}", prompt.prompt_display());
            if std::io::stdout().flush().is_err() {
                println!()
            };
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("failed to read user input: {e}"))?;
            println!();
            Ok::<String, String>(input)
        });

        let input = thread_handle
            .join()
            .map_err(|e| format!("failed to join prompt thread: {e:#?}"))??;

        Ok(input)
    }

    pub async fn walkthrough(&mut self) -> Result<(), String> {
        // ask for model preference
        let input = Self::prompt_user_and(format!(
            "Enter model to use for linting {} ",
            self.model().default_display()
        ))?;
        if !input.trim().is_empty() {
            self.set_model(input.trim().to_owned()).await?;
        }

        // ask for max output tokens preference
        let input = Self::prompt_user_and(format!(
            "Enter max output token usage for each lint request {} ",
            self.max_tokens().to_string().default_display()
        ))?;
        if !input.trim().is_empty() {
            let input = input
                .trim()
                .parse()
                .map_err(|e| format!("error parsing max token usage input: {e}"))?;
            self.set_max_tokens(input).await?;
        }

        // ask for api key
        // TODO: handle token already exists, and instruct user where to get token
        let (api_key_prompt_action, api_key_prompt_suffix) = match get_api_key_fallable() {
            Ok(key) => ("change", key),
            _ => ("set", String::from("undefined/inaccessible")),
        };
        let  input = Self::prompt_user_and(format!("Would you like instructions to {api_key_prompt_action} your OPENROUTER_API_KEY environment variable (currently {api_key_prompt_suffix})? (yes/no) {} ", "no".default_display()))?.to_lowercase();
        let input = input.trim();

        if input == "y" || input == "yes" {
            let input = Self::prompt_user_and(format!(
                "Enter your new OPENROUTER_API_KEY {}",
                if api_key_prompt_suffix == "undefined/inaccessible" {
                    format!("(currently {api_key_prompt_suffix})")
                } else {
                    format!(
                        "(you can signup and get one at {})",
                        OPENROUTER_API_KEY_DASH_URL.path_display()
                    )
                }
            ))?;
            Self::print_api_key_env_var_configuration_instructions(input.trim());
        }

        println!("\n{}\n", "configuration complete!".success_display());
        Ok(())
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn max_tokens(&self) -> u32 {
        self.max_tokens
    }

    async fn update_config_file(&mut self) -> Result<(), String> {
        let config_path = self.path.clone().ok_or("no config path set")?;
        let contents = serde_json::to_string(&self).map_err(|e| e.to_string())?;
        tokio::fs::write(config_path, contents)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn set_model(&mut self, model: String) -> Result<(), String> {
        self.model = model;
        self.update_config_file().await
    }

    pub async fn set_max_tokens(&mut self, max_tokens: u32) -> Result<(), String> {
        self.max_tokens = max_tokens;
        self.update_config_file().await
    }

    fn print_api_key_env_var_configuration_instructions(api_key: &str) {
        match std::env::consts::OS {
            "windows" => {
                println!(
                    "{}",
                    "Run this command in PowerShell (run as Administrator if required):\n"
                        .normal_display()
                );
                println!(
                    "{}",
                    format!("setx {OPENROUTER_API_KEY_VARNAME} \"{api_key}\"\n").info_display()
                );
                println!("{}","This saves your API key as an environment variable so that {CRATE_NAME} can authenticate your lint requests. \nRestart your terminal after running it.".normal_display());
            }
            _ => {
                println!(
                    "{}",
                    "Run this command in your terminal:\n".normal_display()
                );
                println!(
                    "{}",
                    format!(
                        "echo 'export {OPENROUTER_API_KEY_VARNAME}=\"{api_key}\"' >> ~/<your_config_file>\n"
                    ).info_display()
                );
                println!(
                    "{}",
                    "Replace <your_config_file> with your actual shell config file (e.g. .bashrc, .zshrc).".normal_display()
                );
                println!(
                    "{}",
                    "This permanently sets your API key so that {CRATE_NAME} can authenticate your lint requests.".normal_display()
                );
                println!(
                    "{}",
                    "Finally, restart the terminal or (reload your shell: source ~/<your_config_file>)".info_display()
                );
            }
        }
    }
}
