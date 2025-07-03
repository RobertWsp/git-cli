use clap::{Arg, Command};
use log::{info, warn, error, debug};

mod emojis;
mod utils;
mod errors;
mod config;
mod git;
mod ui;
mod validation;

use errors::{Result, GitCliError};
use config::Config;
use git::GitService;
use ui::UIService;

#[derive(Debug)]
struct AppConfig {
    debug: bool,
    non_interactive: bool,
    emoji: Option<String>,
    title: Option<String>,
    body: Option<String>,
}

struct GitWorkflow {
    config: Config,
    app_config: AppConfig,
    git_service: GitService,
    ui_service: UIService,
}

impl GitWorkflow {
    fn new(app_config: AppConfig) -> Result<Self> {
        let config = Config::load()?;
        let git_service = GitService::new(app_config.debug);
        let ui_service = UIService::new(config.clone());

        Ok(Self {
            config,
            app_config,
            git_service,
            ui_service,
        })
    }

    async fn execute(&self) -> Result<()> {
        info!("Starting git-cli workflow");

        // Verify git repository
        self.git_service.verify_git_initialized()?;

        // Get changes
        let changes = self.git_service.get_status()?;
        if changes.is_empty() {
            self.ui_service.show_info("No changes to commit.");
            return Ok(());
        }

        self.ui_service.show_changes(&changes);

        // Stage files
        let selected_files = self.stage_files(&changes).await?;

        // Get commit details
        let (emoji, title, body) = self.get_commit_details(&changes).await?;

        // Create commit message
        let commit_title = format!("{} {}", emoji.emoji, title);
        
        // Attempt commit
        let commit_successful = self.attempt_commit(&commit_title, body.as_deref(), &selected_files).await?;
        
        if !commit_successful {
            return Err(GitCliError::GitCommandFailed("Commit failed".to_string()));
        }

        self.ui_service.show_success(&format!("Successfully committed with emoji: {}", emoji.emoji));

        // Handle remote operations
        self.handle_remote_operations().await?;

        // Show recent commits
        self.show_commit_summary().await?;

        Ok(())
    }

    async fn stage_files(&self, changes: &[git::Change]) -> Result<Vec<String>> {
        let (add_all, selected_files) = if self.app_config.non_interactive {
            (true, changes.iter().map(|c| c.value.clone()).collect())
        } else {
            let add_all = self.ui_service.confirm_add_all_files()?;
            let files = if add_all {
                changes.iter().map(|c| c.value.clone()).collect()
            } else {
                self.ui_service.select_files_to_commit(changes)?
            };
            (add_all, files)
        };

        if selected_files.is_empty() {
            return Err(GitCliError::NoChanges);
        }

        // Stage files
        if add_all {
            self.git_service.add_files(&[])?; // Empty slice means add all
        } else {
            self.git_service.add_files(&selected_files)?;
        }

        self.ui_service.show_info(&format!("Staged {} files", selected_files.len()));
        Ok(selected_files)
    }

    async fn get_commit_details(&self, changes: &[git::Change]) -> Result<(emojis::Emoji, String, Option<String>)> {
        let emojis_object = emojis::get_emojis()?;

        let emoji = if let Some(emoji_str) = &self.app_config.emoji {
            emojis_object
                .emojis
                .iter()
                .find(|e| e.emoji == *emoji_str || e.code == *emoji_str)
                .cloned()
                .ok_or(GitCliError::InvalidEmoji)?
        } else {
            self.ui_service.select_emoji(&emojis_object)?
        };

        let title = if let Some(title) = &self.app_config.title {
            title.clone()
        } else {
            self.ui_service.get_commit_title(changes)?
        };

        let body = if let Some(body) = &self.app_config.body {
            Some(body.clone())
        } else if !self.app_config.non_interactive {
            self.ui_service.get_commit_message()?
        } else {
            None
        };

        Ok((emoji, title, body))
    }

    async fn attempt_commit(&self, title: &str, body: Option<&str>, selected_files: &[String]) -> Result<bool> {
        debug!("Attempting commit with title: {}", title);
        
        let success = self.git_service.commit(title, body)?;
        
        if !success {
            warn!("Initial commit failed, checking for pre-commit hooks");
            
            if self.config.hooks.retry_on_failure {
                self.ui_service.show_warning("Pre-commit hook failed. Re-staging files and retrying...");
                
                // Re-stage files
                self.git_service.add_files(selected_files)?;
                self.ui_service.show_success("Successfully re-staged changes");
                
                // Retry commit
                let retry_success = self.git_service.commit(title, body)?;
                if !retry_success {
                    self.ui_service.show_error("Commit failed after retry");
                    return Ok(false);
                }
                
                self.ui_service.show_success("Commit successful after retry");
                return Ok(true);
            }
        }
        
        Ok(success)
    }

    async fn handle_remote_operations(&self) -> Result<()> {
        let branch = self.git_service.get_current_branch()?;
        self.ui_service.show_info(&format!("Current branch: {}", branch));

        // Fetch remote changes
        if let Err(e) = self.git_service.fetch_origin(&branch) {
            warn!("Failed to fetch from remote: {}", e);
            return Ok(()); // Continue even if fetch fails
        }

        // Check for remote changes
        match self.git_service.has_remote_changes(&branch) {
            Ok(true) => {
                self.ui_service.show_info("There are changes to pull from the remote repository.");
                
                // Try to pull with rebase if configured
                let use_rebase = true; // Could be made configurable
                match self.git_service.pull(&branch, use_rebase) {
                    Ok(()) => {
                        self.ui_service.show_success("Successfully pulled changes from remote");
                    }
                    Err(_) => {
                        self.ui_service.show_warning("Pull failed, trying with stash...");
                        
                        // Stash, pull, then pop
                        self.git_service.stash()?;
                        match self.git_service.pull(&branch, use_rebase) {
                            Ok(()) => {
                                self.ui_service.show_success("Successfully pulled changes");
                                match self.git_service.stash_pop() {
                                    Ok(()) => self.ui_service.show_success("Successfully restored stashed changes"),
                                    Err(e) => self.ui_service.show_warning(&format!("Failed to restore stash: {}", e)),
                                }
                            }
                            Err(e) => {
                                self.ui_service.show_error(&format!("Failed to pull: {}", e));
                            }
                        }
                    }
                }
            }
            Ok(false) => {
                self.ui_service.show_info("No changes to pull from remote");
            }
            Err(e) => {
                warn!("Failed to check remote changes: {}", e);
            }
        }

        // Ask user if they want to push
        if !self.app_config.non_interactive {
            let should_push = self.ui_service.confirm_push()?;
            if should_push {
                match self.git_service.push(&branch) {
                    Ok(()) => {
                        self.ui_service.show_success(&format!("Successfully pushed to origin/{}", branch));
                    }
                    Err(e) => {
                        self.ui_service.show_error(&format!("Failed to push: {}", e));
                    }
                }
            }
        }

        Ok(())
    }

    async fn show_commit_summary(&self) -> Result<()> {
        match self.git_service.get_recent_commits(5) {
            Ok(commits) => {
                self.ui_service.show_recent_commits(&commits);
            }
            Err(e) => {
                warn!("Failed to get recent commits: {}", e);
            }
        }
        Ok(())
    }
}

fn parse_args() -> AppConfig {
    let matches = Command::new("Git CLI with Emojis")
        .version("0.2.0")
        .author("RobertWsp <sousarobert854@gmail.com>")
        .about("A powerful CLI tool for commits with emojis and conventional commit support")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("non-interactive")
                .long("no-interactive")
                .help("Run in non-interactive mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("emoji")
                .long("emoji")
                .help("Emoji to use for commit")
                .value_name("EMOJI")
        )
        .arg(
            Arg::new("title")
                .long("title")
                .help("Commit title")
                .value_name("TITLE")
        )
        .arg(
            Arg::new("body")
                .long("body")
                .help("Commit body/description")
                .value_name("BODY")
        )
        .get_matches();

    AppConfig {
        debug: matches.get_flag("debug"),
        non_interactive: matches.get_flag("non-interactive"),
        emoji: matches.get_one::<String>("emoji").cloned(),
        title: matches.get_one::<String>("title").cloned(),
        body: matches.get_one::<String>("body").cloned(),
    }
}

#[tokio::main]
async fn main() {
    let app_config = parse_args();
    
    // Initialize logging
    utils::init_logger(app_config.debug);
    
    info!("Git CLI started");
    debug!("App config: {:?}", app_config);

    // Create and execute workflow
    match GitWorkflow::new(app_config) {
        Ok(workflow) => {
            if let Err(e) = workflow.execute().await {
                error!("Workflow failed: {}", e);
                eprintln!("{}", utils::format_error_message(&format!("Error: {}", e)));
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to initialize workflow: {}", e);
            eprintln!("{}", utils::format_error_message(&format!("Failed to start: {}", e)));
            std::process::exit(1);
        }
    }

    info!("Git CLI completed successfully");
}
