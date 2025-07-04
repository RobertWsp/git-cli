use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitCliError {
    #[error("Git repository not found")]
    NotGitRepo,
    
    #[error("Git command failed: {0}")]
    GitCommandFailed(String),
    
    #[error("User cancelled operation")]
    UserCancelled,
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::InquireError),
    
    #[error("Invalid emoji selected")]
    InvalidEmoji,
    
    #[error("No changes to commit")]
    NoChanges,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, GitCliError>;

#[derive(Debug)]
pub enum ValidationError {
    TitleTooLong,
    TitleShouldStartLowercase,
    BodyTooLong,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TitleTooLong => write!(f, "Commit title is too long (max 50 characters)"),
            ValidationError::TitleShouldStartLowercase => write!(f, "Title should start with lowercase letter"),
            ValidationError::BodyTooLong => write!(f, "Body lines should be 72 characters or less"),
        }
    }
}
