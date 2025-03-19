use clap::{Arg, Command};
use inquire::{InquireError, Select};
use std::io::BufRead;
mod emojis;
mod utils;

#[derive(Debug, Clone)]
struct Change {
    color: String,
    change_type: String,
    value: String,
}

fn selected_emoji(emojis_object: emojis::EmojisObject, selected: String) -> emojis::Emoji {
    let selected_emoji = emojis_object
        .emojis
        .iter()
        .find(|emoji| selected.starts_with(&emoji.emoji))
        .expect(&utils::format_error_message("Invalid emoji selected."));

    return (*selected_emoji).clone();
}

fn verify_git_initialized() {
    // Run 'git rev-parse --is-inside-work-tree' to check if the current directory is a git repository
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect(&utils::format_error_message(
            "Failed to execute git command",
        ));

    if !output.stdout.starts_with(b"true") {
        panic!("Not a git repository.");
    }
}

fn content_to_commit() -> Vec<Change> {
    // Run 'git status --porcelain' to check for uncommitted changes
    let output = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect(&utils::format_error_message(
            "Failed to execute git command",
        ));

    let status_output = String::from_utf8_lossy(&output.stdout);

    println!();

    let mut changes = Vec::new();

    for line in status_output.lines() {
        let line_parsed = line.trim().split_whitespace().last().unwrap().to_string();

        let change = if line.find("A ").is_some() {
            Change {
                color: "\x1b[0;32m".to_string(), // Green for added files
                change_type: "Added".to_string(),
                value: line_parsed,
            }
        } else if line.find("M ").is_some() {
            Change {
                color: "\x1b[0;33m".to_string(), // Yellow for modified files
                change_type: "Modified".to_string(),
                value: line_parsed,
            }
        } else if line.find("D ").is_some() {
            Change {
                color: "\x1b[0;31m".to_string(), // Red for deleted files
                change_type: "Deleted".to_string(),
                value: line_parsed,
            }
        } else if line.find("R ").is_some() {
            Change {
                color: "\x1b[0;34m".to_string(), // Blue for renamed files
                change_type: "Renamed".to_string(),
                value: line_parsed,
            }
        } else if line.find("??").is_some() {
            Change {
                color: "\x1b[0;35m".to_string(), // Magenta for untracked files
                change_type: "Untracked".to_string(),
                value: line_parsed,
            }
        } else {
            continue;
        };

        changes.push(change);
    }

    println!();

    return changes;
}

fn run_command_stream(
    command: &str,
    args: Vec<&str>,
    error_message: &str,
) -> std::process::ExitStatus {
    let mut child = std::process::Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect(&utils::format_error_message(error_message));

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_reader = std::io::BufReader::new(stdout);
    let stderr_reader = std::io::BufReader::new(stderr);

    let stdout_thread = std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line);
            }
        }
    });

    let status = child.wait().expect("Failed to wait on child");

    stdout_thread.join().expect("Failed to join stdout thread");
    stderr_thread.join().expect("Failed to join stderr thread");

    return status;
}

fn format_string_to_title(title: String) -> String {
    let mut chars = title.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn main() {
    let matches = Command::new("Emoji Commit")
        .version("1.0")
        .author("RobertWsp <sousarobert854@gmail.com>")
        .about("A simple CLI tool to commit with emojis.")
        .arg(Arg::new("debug").short('d').long("debug").required(false))
        .get_matches();

    verify_git_initialized();

    let debug = matches.contains_id("debug");

    if debug {
        println!("Debug mode enabled.");
    }

    let changes = content_to_commit();

    if !changes.is_empty() {
        let changes_to_commit: Result<Vec<String>, InquireError> = inquire::MultiSelect::new(
            "Select changes to add to the commit:",
            changes
                .iter()
                .map(|change| format!("{}: {}{}", change.change_type, change.color, change.value))
                .collect(),
        )
        .prompt();

        let changes_to_commit = match changes_to_commit {
            Ok(changes) => changes,
            Err(e) => {
                println!("{}", utils::format_error_message(&format!("Error: {}", e)));
                return;
            }
        };

        let changes_to_commit: Vec<Change> = changes
            .iter()
            .filter(|change| {
                changes_to_commit.contains(&format!(
                    "{}: {}{}",
                    change.change_type, change.color, change.value
                ))
            })
            .cloned()
            .collect();

        for change in changes_to_commit {
            let file_path = change.value.clone();

            let add_to_commit_result = std::process::Command::new("git")
                .arg("add")
                .arg(&file_path)
                .output()
                .expect(&utils::format_error_message("Failed to stage changes"));

            if !add_to_commit_result.status.success() {
                if debug {
                    eprintln!("{}", String::from_utf8_lossy(&add_to_commit_result.stderr));
                }
                println!(
                    "{}",
                    utils::format_error_message(&format!(
                        "Error: Failed to stage changes for file: {}",
                        file_path
                    ))
                );
            }
        }
    }

    let emojis_object =
        emojis::get_emojis().expect(&utils::format_error_message("Failed to load emojis."));

    let answer: Result<String, InquireError> = Select::new(
        "Select an emoji for your commit message?",
        emojis_object
            .emojis
            .iter()
            .map(|emoji| format!("{} - {}", emoji.emoji, emoji.description))
            .collect::<Vec<String>>(),
    )
    .prompt();

    let selected_emoji = match answer {
        Ok(selected) => selected_emoji(emojis_object, selected),
        Err(e) => {
            println!("{}", utils::format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let commit_title: Result<String, InquireError> =
        inquire::CustomType::<String>::new("Enter commit title: ")
            .with_parser(&|input| Ok(format_string_to_title(input.to_string())))
            .prompt();

    let commit_title = match commit_title {
        Ok(title) => title,
        Err(e) => {
            println!("{}", utils::format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let commit_message: Result<String, InquireError> =
        inquire::CustomType::<String>::new("Enter commit message: ")
            .with_parser(&|message| Ok(format_string_to_title(message.to_string())))
            .prompt();

    let commit_message = match commit_message {
        Ok(message) => message,
        Err(e) => {
            println!("{}", utils::format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let mut args = vec!["commit"];

    args.push("-m");

    let commit_title_with_emoji = format!("{} {}", selected_emoji.emoji, commit_title);

    args.push(&commit_title_with_emoji);

    let formatted_commit_message;

    if commit_message.len() > 0 {
        formatted_commit_message = format!("-m \"{}\"", commit_message);

        args.push(&formatted_commit_message);
    }

    let status = run_command_stream("git", args, "Failed to commit changes");

    if status.success() {
        println!(
            "\x1b[0;32mSuccessfully committed with emoji: {}\x1b[0m",
            selected_emoji.emoji
        );
    } else {
        println!(
            "{}",
            utils::format_error_message(&format!(
                "Error: Failed to commit with emoji: {}",
                selected_emoji.emoji
            ))
        );
    }

    // Get the current branch name
    let branch_output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect(&utils::format_error_message("Failed to get current branch"));

    let branch_name = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    // Check if there are any changes to pull from the remote repository
    let fetch_status = std::process::Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(&branch_name)
        .output()
        .expect(&utils::format_error_message(
            "Failed to fetch changes from remote",
        ));

    if !fetch_status.status.success() {
        println!(
            "{}",
            utils::format_error_message("Error: Failed to fetch changes from remote repository")
        );
    } else {
        let local_commit = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .expect(&utils::format_error_message(
                "Failed to get local commit hash",
            ));

        let remote_commit = std::process::Command::new("git")
            .arg("rev-parse")
            .arg(format!("origin/{}", branch_name))
            .output()
            .expect(&utils::format_error_message(
                "Failed to get remote commit hash",
            ));

        if local_commit.stdout != remote_commit.stdout {
            println!("There are changes to pull from the remote repository.");

            let git_pull_args = vec!["pull", "origin", &branch_name];

            let status =
                run_command_stream("git", git_pull_args, "Failed to pull changes from remote");

            if !status.success() {
                println!(
                    "{}",
                    utils::format_error_message(
                        "Error: Failed to pull changes from remote repository"
                    )
                );

                let stash_status = std::process::Command::new("git")
                    .arg("stash")
                    .output()
                    .expect(&utils::format_error_message("Failed to stash changes"));

                if stash_status.status.success() {
                    println!("\x1b[0;32mSuccessfully stashed changes\x1b[0m");

                    let git_pull_args = vec!["pull", "origin", &branch_name];

                    let status = run_command_stream(
                        "git",
                        git_pull_args,
                        "Failed to pull changes from remote",
                    );

                    if status.success() {
                        println!(
                            "\x1b[0;32mSuccessfully pulled changes from remote repository\x1b[0m"
                        );

                        let stash_pop_status = std::process::Command::new("git")
                            .arg("stash")
                            .arg("pop")
                            .output()
                            .expect(&utils::format_error_message(
                                "Failed to pop stashed changes",
                            ));

                        if stash_pop_status.status.success() {
                            println!("\x1b[0;32mSuccessfully popped stashed changes\x1b[0m");
                        } else {
                            println!(
                                "{}",
                                utils::format_error_message("Error: Failed to pop stashed changes")
                            );
                        }
                    } else {
                        println!(
                            "{}",
                            utils::format_error_message(
                                "Error: Failed to pull changes from remote repository"
                            )
                        );
                    }
                } else {
                    println!(
                        "{}",
                        utils::format_error_message("Error: Failed to stash changes")
                    );
                }
            }
        } else {
            println!("No changes to pull from the remote repository.");
        }
    }

    let log_output = std::process::Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .expect(&utils::format_error_message(
            "Failed to retrieve commit log",
        ));

    let log_output = String::from_utf8_lossy(&log_output.stdout);
    let first_five_commits: Vec<&str> = log_output.lines().take(5).collect();
    println!("Current commits:\n{}", first_five_commits.join("\n"));

    println!("Current branch: {}", branch_name);

    let push_to_remote: Result<bool, InquireError> =
        inquire::Confirm::new("Do you want to push the commits to the remote repository?").prompt();

    let push_to_remote = match push_to_remote {
        Ok(push) => push,
        Err(e) => {
            println!("{}", utils::format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    if push_to_remote {
        let push_status = std::process::Command::new("git")
            .arg("push")
            .arg("origin")
            .arg(&branch_name)
            .output()
            .expect(&utils::format_error_message("Failed to push commits"));

        if push_status.status.success() {
            println!(
                "\x1b[0;32mSuccessfully pushed commits to remote repository on branch: {}\x1b[0m",
                branch_name
            );
        } else {
            println!(
                "{}",
                utils::format_error_message("Error: Failed to push commits to remote repository")
            );
        }
    }
}
