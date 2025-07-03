use crate::errors::{Result, GitCliError, ValidationError};
use crate::emojis::{Emoji, EmojisObject};
use crate::git::Change;
use crate::config::Config;
use inquire::{Select, MultiSelect, Confirm, Text};
use log::debug;

pub struct UIService {
    config: Config,
}

impl UIService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn confirm_add_all_files(&self) -> Result<bool> {
        let result = Confirm::new("Do you want to add all changes to the commit?")
            .prompt()
            .map_err(GitCliError::InquireError)?;
        
        debug!("User chose to add all files: {}", result);
        Ok(result)
    }

    pub fn select_files_to_commit(&self, changes: &[Change]) -> Result<Vec<String>> {
        let options: Vec<String> = changes
            .iter()
            .map(|change| {
                format!(
                    "{}{}: {}\x1b[0m",
                    change.color, change.change_type, change.value
                )
            })
            .collect();

        let selected = MultiSelect::new("Select changes to add to the commit:", options)
            .prompt()
            .map_err(GitCliError::InquireError)?;

        let selected_files: Vec<String> = changes
            .iter()
            .filter(|change| {
                selected.contains(&format!(
                    "{}{}: {}\x1b[0m",
                    change.color, change.change_type, change.value
                ))
            })
            .map(|change| change.value.clone())
            .collect();

        debug!("User selected {} files", selected_files.len());
        Ok(selected_files)
    }

    pub fn select_emoji(&self, emojis_object: &EmojisObject) -> Result<Emoji> {
        let options: Vec<String> = emojis_object
            .emojis
            .iter()
            .map(|emoji| format!("{} - {}", emoji.emoji, emoji.description))
            .collect();

        let selected = Select::new("Select an emoji for your commit message:", options)
            .prompt()
            .map_err(GitCliError::InquireError)?;

        let selected_emoji = emojis_object
            .emojis
            .iter()
            .find(|emoji| selected.starts_with(&emoji.emoji))
            .ok_or(GitCliError::InvalidEmoji)?;

        debug!("User selected emoji: {}", selected_emoji.emoji);
        Ok(selected_emoji.clone())
    }

    pub fn get_commit_title(&self, changes: &[crate::git::Change]) -> Result<String> {
        loop {
            let placeholder = self.generate_smart_placeholder(changes);
            
            let help_message = if self.config.commit.enforce_conventional {
                format!(
                    "Use conventional commit format: type(scope): description\n\
                     Types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert\n\
                     Example: feat: add user authentication\n\
                     Max length: {} characters",
                    self.config.commit.max_title_length
                )
            } else {
                format!("Max length: {} characters", self.config.commit.max_title_length)
            };

            let title = Text::new("Enter commit title:")
                .with_placeholder(&placeholder)
                .with_help_message(&help_message)
                .prompt()
                .map_err(GitCliError::InquireError)?;

            let formatted_title = if self.config.commit.auto_capitalize_title {
                self.format_string_to_title(title)
            } else {
                title
            };

            // Validate title
            match self.validate_commit_title(&formatted_title) {
                Ok(_) => {
                    debug!("User entered valid title: {}", formatted_title);
                    return Ok(formatted_title);
                },
                Err(e) => {
                    log::warn!("Validation failed: {}", e);
                    println!("\n❌ {}", e);
                    println!("Please try again.\n");
                    continue;
                }
            }
        }
    }

    pub fn get_commit_message(&self) -> Result<Option<String>> {
        let message = Text::new("Enter commit message (optional):")
            .with_default("")
            .prompt()
            .map_err(GitCliError::InquireError)?;

        if message.trim().is_empty() {
            return Ok(None);
        }

        let formatted_message = if self.config.commit.auto_capitalize_title {
            self.format_string_to_title(message)
        } else {
            message
        };

        // Validate message
        self.validate_commit_body(&formatted_message)?;

        debug!("User entered message: {}", formatted_message);
        Ok(Some(formatted_message))
    }

    pub fn confirm_push(&self) -> Result<bool> {
        if !self.config.general.confirm_before_push {
            return Ok(self.config.general.auto_push);
        }

        let result = Confirm::new("Do you want to push the commits to the remote repository?")
            .prompt()
            .map_err(GitCliError::InquireError)?;
        
        debug!("User chose to push: {}", result);
        Ok(result)
    }

    pub fn show_changes(&self, changes: &[Change]) {
        if changes.is_empty() {
            println!("No changes found.");
            return;
        }

        println!("\nChanges to commit:");
        for change in changes {
            println!(
                "  {}{}: {}\x1b[0m",
                change.color, change.change_type, change.value
            );
        }
        println!();
    }

    pub fn show_recent_commits(&self, commits: &[String]) {
        if commits.is_empty() {
            return;
        }

        println!("Recent commits:");
        for commit in commits {
            println!("  {}", commit);
        }
        println!();
    }

    pub fn show_success(&self, message: &str) {
        println!("\x1b[0;32m{}\x1b[0m", message);
    }

    pub fn show_error(&self, message: &str) {
        println!("\x1b[0;31m{}\x1b[0m", message);
    }

    pub fn show_warning(&self, message: &str) {
        println!("\x1b[0;33m{}\x1b[0m", message);
    }

    pub fn show_info(&self, message: &str) {
        println!("{}", message);
    }

    fn format_string_to_title(&self, input: String) -> String {
        let mut chars = input.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    fn validate_commit_title(&self, title: &str) -> Result<()> {
        let mut errors = Vec::new();

        // Check length
        if title.len() > self.config.commit.max_title_length {
            errors.push(format!(
                "Title is {} characters (max {})",
                title.len(),
                self.config.commit.max_title_length
            ));
        }

        // Check conventional commit format
        if self.config.commit.enforce_conventional && !self.is_conventional_commit(title) {
            errors.push(
                "Must follow conventional commit format: type(scope): description\n\
                 Valid types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert\n\
                 Examples:\n\
                 • feat: add user authentication\n\
                 • fix(ui): resolve button alignment\n\
                 • docs: update installation guide".to_string()
            );
        }

        if !errors.is_empty() {
            let error_msg = errors.join("\n\n");
            return Err(GitCliError::ValidationError(error_msg));
        }

        Ok(())
    }

    fn validate_commit_body(&self, body: &str) -> Result<()> {
        let mut errors = Vec::new();

        for line in body.lines() {
            if line.len() > self.config.commit.max_body_length {
                errors.push(ValidationError::BodyTooLong);
                break;
            }
        }

        if !errors.is_empty() {
            let error_msg = errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(GitCliError::ValidationError(error_msg));
        }

        Ok(())
    }

    fn is_conventional_commit(&self, title: &str) -> bool {
        let conventional_types = [
            "feat", "fix", "docs", "style", "refactor", 
            "test", "chore", "perf", "ci", "build", "revert"
        ];
        
        debug!("Checking conventional commit format for: '{}'", title);
        
        for commit_type in &conventional_types {
            let pattern1 = format!("{}: ", commit_type);
            let pattern2 = format!("{}(", commit_type);
            
            debug!("Checking patterns: '{}' or '{}'", pattern1, pattern2);
            
            if title.starts_with(&pattern1) || title.starts_with(&pattern2) {
                debug!("Title matches conventional commit format!");
                return true;
            }
        }
        
        debug!("Title does NOT match conventional commit format");
        false
    }

    fn generate_smart_placeholder(&self, changes: &[crate::git::Change]) -> String {
        // Analyze changes to suggest appropriate commit type
        let mut has_new_files = false;
        let mut has_docs = false;
        let mut has_tests = false;
        let mut has_config = false;
        let mut has_ui = false;
        let mut has_deps = false;
        
        for change in changes {
            let file_lower = change.value.to_lowercase();
            
            if change.change_type == "Added" {
                has_new_files = true;
            }
            
            if file_lower.ends_with(".md") || file_lower.ends_with(".rst") || file_lower.contains("readme") {
                has_docs = true;
            } else if file_lower.contains("test") || file_lower.contains("spec") {
                has_tests = true;
            } else if file_lower.ends_with(".toml") || file_lower.ends_with(".json") || file_lower.ends_with(".yml") {
                has_config = true;
            } else if file_lower.ends_with(".css") || file_lower.ends_with(".scss") || file_lower.contains("ui") {
                has_ui = true;
            } else if file_lower.contains("cargo.toml") || file_lower.contains("package.json") {
                has_deps = true;
            }
        }
        
        // Generate suggestion based on analysis
        if self.config.commit.enforce_conventional {
            if has_deps {
                "chore: update dependencies".to_string()
            } else if has_docs && !has_tests && !has_new_files {
                "docs: update documentation".to_string()
            } else if has_tests && !has_new_files {
                "test: add unit tests".to_string()
            } else if has_config {
                "chore: update configuration".to_string()
            } else if has_ui {
                "style: improve UI components".to_string()
            } else if has_new_files {
                "feat: add new feature".to_string()
            } else {
                "fix: resolve issue".to_string()
            }
        } else {
            if has_deps {
                "update dependencies"
            } else if has_docs {
                "update documentation"
            } else if has_tests {
                "add tests"
            } else if has_config {
                "update configuration"
            } else if has_ui {
                "improve styling"
            } else if has_new_files {
                "add new feature"
            } else {
                "fix issue"
            }.to_string()
        }
    }
}
