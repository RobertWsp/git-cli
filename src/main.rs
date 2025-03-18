use clap::{Arg, Command};
use inquire::{InquireError, Select};
use std::io::BufRead;
mod emojis;
mod utils;

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

fn content_to_commit() -> bool {
    // Run 'git status --porcelain' to check for uncommitted changes
    let output = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect(&utils::format_error_message(
            "Failed to execute git command",
        ));

    let status_output = String::from_utf8_lossy(&output.stdout);
    let staged_changes = status_output.lines().any(|line| {
        line.find("A ").is_some()
            || line.find("M ").is_some()
            || line.find("D ").is_some()
            || line.find("R ").is_some()
    });

    println!();

    for line in status_output.lines() {
        if line.find("A ").is_some() {
            println!("\x1b[0;32m{}\x1b[0m", line); // Green for added files
        } else if line.find("M ").is_some() {
            println!("\x1b[0;33m{}\x1b[0m", line); // Yellow for modified files
        } else if line.find("D ").is_some() {
            println!("\x1b[0;31m{}\x1b[0m", line); // Red for deleted files
        } else if line.find("R ").is_some() {
            println!("\x1b[0;34m{}\x1b[0m", line); // Blue for renamed files
        }
    }

    println!();

    return staged_changes;
}

fn main() {
    let matches = Command::new("Emoji Commit")
        .version("1.0")
        .author("RobertWsp <sousarobert854@gmail.com>")
        .about("A simple CLI tool to commit with emojis.")
        .arg(Arg::new("debug").short('d').long("debug").required(false))
        .get_matches();

    verify_git_initialized();

    let has_content = content_to_commit();

    if has_content {
        let add_to_commit: Result<bool, InquireError> = inquire::Confirm::new(
            "There are changes to commit. Do you want to add them to the commit?",
        )
        .prompt();

        let add_to_commit = match add_to_commit {
            Ok(add) => add,
            Err(e) => {
                println!("{}", utils::format_error_message(&format!("Error: {}", e)));
                return;
            }
        };

        if add_to_commit {
            std::process::Command::new("git")
                .arg("add")
                .arg("--all")
                .output()
                .expect(&utils::format_error_message("Failed to stage changes"));
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
        inquire::Text::new("Enter commit title: ").prompt();

    let commit_title = match commit_title {
        Ok(title) => title,
        Err(e) => {
            println!("{}", utils::format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let commit_message: Result<String, InquireError> =
        inquire::Text::new("Enter commit message: ").prompt();

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

    let mut child = std::process::Command::new("git")
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect(&utils::format_error_message("Failed to execute command"));

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

    if matches.contains_id("debug") {
        println!("Debug mode enabled.");
    }
}
