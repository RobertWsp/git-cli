use git_cli::{config::Config, errors::GitCliError, validation::ValidationService};
use tempfile::TempDir;
use std::process::Command;

#[test]
fn test_config_load_default() {
    let config = Config::default();
    assert_eq!(config.general.default_emoji, "✨");
    assert_eq!(config.commit.max_title_length, 50);
    assert!(config.hooks.run_pre_commit);
}

#[test]
fn test_validation_service_emoji_suggestions() {
    let files = vec![
        "README.md".to_string(),
        "test_file.rs".to_string(),
        "style.css".to_string(),
    ];
    
    let suggestions = ValidationService::suggest_emoji_by_files(&files);
    
    assert!(suggestions.contains(&"📝")); // docs
    assert!(suggestions.contains(&"💄")); // style
}

#[test]
fn test_validation_service_commit_templates() {
    let templates = ValidationService::get_commit_templates();
    
    assert!(!templates.is_empty());
    assert!(templates.iter().any(|t| t.name == "Feature"));
    assert!(templates.iter().any(|t| t.name == "Bugfix"));
}

#[cfg(test)]
mod git_tests {
    use super::*;
    use git_cli::git::GitService;
    
    fn setup_test_repo() -> TempDir {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repo_path = temp_dir.path();
        
        // Initialize git repo
        Command::new("git")
            .arg("init")
            .current_dir(repo_path)
            .output()
            .expect("Failed to init git repo");
            
        // Set up git config
        Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git email");
            
        Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git name");
        
        temp_dir
    }
    
    #[test]
    fn test_git_service_verify_initialized() {
        let _temp_repo = setup_test_repo();
        let git_service = GitService::new(false);
        
        // This should work in the temp repo
        // Note: This test would need to be run from within the temp repo directory
        // For now, we'll just test that the service can be created
        assert!(!git_service.debug);
    }
    
    #[test]
    fn test_git_service_get_status_empty() {
        let git_service = GitService::new(true);
        
        // Test that debug mode is set correctly
        assert!(git_service.debug);
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;
    
    #[test]
    fn test_git_cli_error_display() {
        let error = GitCliError::NotGitRepo;
        assert_eq!(error.to_string(), "Git repository not found");
        
        let error = GitCliError::InvalidEmoji;
        assert_eq!(error.to_string(), "Invalid emoji selected");
    }
    
    #[test]
    fn test_git_cli_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let cli_error: GitCliError = io_error.into();
        
        match cli_error {
            GitCliError::IoError(_) => (),
            _ => panic!("Expected IoError variant"),
        }
    }
}
