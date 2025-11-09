use serde::{Deserialize, Serialize};
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub github: GitHubConfig,
    pub application: ApplicationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub repository_url: String,
    pub branch: String,
    pub webhook_secret: String,
    pub api_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub working_directory: String,
    pub start_command: Vec<String>,
    pub virtual_env_path: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        // TODO: Implement configuration loading from file/env
        todo!("Configuration loading not implemented")
    }
}
