use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("Webhook validation failed: {0}")]
    WebhookValidation(String),

    #[error("GitHub API error: {0}")]
    GitHubApi(String),

    #[error("Package manager error: {0}")]
    PackageManager(String),

    #[error("Application management error: {0}")]
    AppManager(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, MonitorError>;
