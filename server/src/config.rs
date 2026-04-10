use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::CRATE_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: Option<PathBuf>,
    model: String,
    max_tokens: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: None,
            max_tokens: 500,
            model: "nvidia/nemotron-3-super-120b-a12b:free".to_owned(),
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

    pub fn walkthrough() {
        // ask for api key
        // ask for model preference
        // ask for max output tokens preference
        todo!()
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
}
