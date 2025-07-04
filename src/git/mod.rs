use crate::errors::{Result, GitCliError};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use log::{debug, info, warn};

#[derive(Debug, Clone)]
pub struct Change {
    pub color: String,
    pub change_type: String,
    pub value: String,
}

pub struct GitService {
    pub debug: bool,
}

impl GitService {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    pub fn verify_git_initialized(&self) -> Result<()> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to execute git command: {}", e)))?;

        if !output.stdout.starts_with(b"true") {
            return Err(GitCliError::NotGitRepo);
        }

        debug!("Git repository verified");
        Ok(())
    }

    pub fn get_status(&self) -> Result<Vec<Change>> {
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to get git status: {}", e)))?;

        let status_output = String::from_utf8_lossy(&output.stdout);
        let mut changes = Vec::new();

        for line in status_output.lines() {
            if line.is_empty() || line.len() < 3 {
                continue;
            }
            
            let status_chars: Vec<char> = line.chars().collect();
            let index_status = status_chars[0];
            let working_tree_status = status_chars[1];
            let filename = line[3..].to_string();

            // Determine the primary change type based on both index and working tree status
            let change = if index_status == '?' && working_tree_status == '?' {
                Change {
                    color: "\x1b[0;35m".to_string(), // Magenta for untracked files
                    change_type: "Untracked".to_string(),
                    value: filename,
                }
            } else if index_status == 'A' || working_tree_status == 'A' {
                Change {
                    color: "\x1b[0;32m".to_string(), // Green for added files
                    change_type: "Added".to_string(),
                    value: filename,
                }
            } else if index_status == 'M' || working_tree_status == 'M' {
                Change {
                    color: "\x1b[0;33m".to_string(), // Yellow for modified files
                    change_type: "Modified".to_string(),
                    value: filename,
                }
            } else if index_status == 'D' || working_tree_status == 'D' {
                Change {
                    color: "\x1b[0;31m".to_string(), // Red for deleted files
                    change_type: "Deleted".to_string(),
                    value: filename,
                }
            } else if index_status == 'R' || working_tree_status == 'R' {
                Change {
                    color: "\x1b[0;34m".to_string(), // Blue for renamed files
                    change_type: "Renamed".to_string(),
                    value: filename,
                }
            } else if index_status == 'C' || working_tree_status == 'C' {
                Change {
                    color: "\x1b[0;36m".to_string(), // Cyan for copied files
                    change_type: "Copied".to_string(),
                    value: filename,
                }
            } else {
                // For any other status, treat as modified
                Change {
                    color: "\x1b[0;33m".to_string(), // Yellow for modified files
                    change_type: "Modified".to_string(),
                    value: filename,
                }
            };

            changes.push(change);
        }

        debug!("Found {} changes", changes.len());
        Ok(changes)
    }

    pub fn add_files(&self, files: &[String]) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.arg("add");
        
        if files.is_empty() {
            cmd.arg(".");
        } else {
            cmd.args(files);
        }

        let output = cmd.output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to stage files: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitCliError::GitCommandFailed(format!("Git add failed: {}", error_msg)));
        }

        info!("Successfully staged {} files", if files.is_empty() { "all".to_string() } else { files.len().to_string() });
        Ok(())
    }

    pub fn run_command_stream(&self, args: Vec<&str>, error_message: &str) -> Result<(String, bool)> {
        let mut child = Command::new("git")
            .args(args.iter().filter(|&arg| !arg.is_empty()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| GitCliError::GitCommandFailed(format!("{}: {}", error_message, e)))?;

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let stdout_thread = std::thread::spawn(move || {
            let mut output = String::new();
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }
            }
            output
        });

        let stderr_thread = std::thread::spawn(move || {
            let mut output = String::new();
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    eprintln!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }
            }
            output
        });

        let status = child.wait()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to wait on child: {}", e)))?;

        let stdout_output = stdout_thread.join().expect("Failed to join stdout thread");
        let stderr_output = stderr_thread.join().expect("Failed to join stderr thread");

        let combined_output = format!("{}\n{}", stdout_output, stderr_output);

        Ok((combined_output, status.success()))
    }

    pub fn commit(&self, title: &str, body: Option<&str>) -> Result<bool> {
        let mut args = vec!["commit", "-m", title];
        
        if let Some(body) = body {
            args.push("-m");
            args.push(body);
        }

        debug!("Running git commit with args: {:?}", args);
        let (output, success) = self.run_command_stream(args, "Failed to commit changes")?;
        
        if success {
            info!("Commit successful");
        } else {
            warn!("Commit failed: {}", output);
        }

        Ok(success)
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to get current branch: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitCliError::GitCommandFailed(format!("Failed to get branch: {}", error_msg)));
        }

        let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        debug!("Current branch: {}", branch_name);
        Ok(branch_name)
    }

    pub fn fetch_origin(&self, branch: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .arg(branch)
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to fetch: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitCliError::GitCommandFailed(format!("Fetch failed: {}", error_msg)));
        }

        debug!("Fetched origin/{}", branch);
        Ok(())
    }

    pub fn has_remote_changes(&self, branch: &str) -> Result<bool> {
        let local_output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to get local commit: {}", e)))?;

        let remote_output = Command::new("git")
            .arg("rev-parse")
            .arg(format!("origin/{}", branch))
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to get remote commit: {}", e)))?;

        Ok(local_output.stdout != remote_output.stdout)
    }

    pub fn pull(&self, branch: &str, use_rebase: bool) -> Result<()> {
        let mut args = vec!["pull"];
        if use_rebase {
            args.push("--rebase");
        }
        args.extend_from_slice(&["origin", branch]);

        let (output, success) = self.run_command_stream(args, "Failed to pull changes")?;
        
        if !success {
            return Err(GitCliError::GitCommandFailed(format!("Pull failed: {}", output)));
        }

        info!("Successfully pulled changes");
        Ok(())
    }

    pub fn push(&self, branch: &str) -> Result<()> {
        let (output, success) = self.run_command_stream(
            vec!["push", "origin", branch], 
            "Failed to push commits"
        )?;
        
        if !success {
            return Err(GitCliError::GitCommandFailed(format!("Push failed: {}", output)));
        }

        info!("Successfully pushed to origin/{}", branch);
        Ok(())
    }

    pub fn stash(&self) -> Result<()> {
        let output = Command::new("git")
            .arg("stash")
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to stash: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitCliError::GitCommandFailed(format!("Stash failed: {}", error_msg)));
        }

        info!("Successfully stashed changes");
        Ok(())
    }

    pub fn stash_pop(&self) -> Result<()> {
        let output = Command::new("git")
            .arg("stash")
            .arg("pop")
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to pop stash: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitCliError::GitCommandFailed(format!("Stash pop failed: {}", error_msg)));
        }

        info!("Successfully popped stashed changes");
        Ok(())
    }

    pub fn get_recent_commits(&self, count: usize) -> Result<Vec<String>> {
        let output = Command::new("git")
            .arg("log")
            .arg("--oneline")
            .arg(format!("-{}", count))
            .output()
            .map_err(|e| GitCliError::GitCommandFailed(format!("Failed to get commits: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitCliError::GitCommandFailed(format!("Get commits failed: {}", error_msg)));
        }

        let log_output = String::from_utf8_lossy(&output.stdout);
        let commits: Vec<String> = log_output.lines().map(|s| s.to_string()).collect();
        
        debug!("Retrieved {} recent commits", commits.len());
        Ok(commits)
    }
}
