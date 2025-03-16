use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Emoji {
    code: String,
    name: String,
    emoji: String,
    entity: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmojisObject {
    emojis: Vec<Emoji>,
}

fn format_error_message(message: &str) -> String {
    return format!("\x1b[0;31m{}\x1b[0m", message);
}

fn panic_error_message(message: &str) {
    panic!("{}", format_error_message(message));
}

fn selected_emoji(emojis_object: EmojisObject, selected: String) -> Emoji {
    let selected_emoji = emojis_object
        .emojis
        .iter()
        .find(|emoji| selected.starts_with(&emoji.emoji))
        .expect(&format_error_message("Invalid emoji selected."));

    return (*selected_emoji).clone();
}

fn verify_git_initialized() {
    // Run 'git rev-parse --is-inside-work-tree' to check if the current directory is a git repository
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect(&format_error_message("Failed to execute git command"));

    if !output.stdout.starts_with(b"true") {
        panic!("Not a git repository.");
    }
}

fn verify_content_to_commit() {
    // Run 'git status --porcelain' to check for uncommitted changes
    let output = std::process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect(&format_error_message("Failed to execute git command"));

    if output.stdout.is_empty() {
        panic_error_message("No changes to commit.");
    }

    let status_output = String::from_utf8_lossy(&output.stdout);
    let staged_changes = status_output
        .lines()
        .any(|line| line.starts_with("A") || line.starts_with("M") || line.starts_with("R"));

    if !staged_changes {
        panic_error_message("No changes to commit.");
    }
}

fn main() {
    verify_git_initialized();
    verify_content_to_commit();

    let emojis_json_path = "emojis.json";

    let emojis_json = std::fs::read_to_string(emojis_json_path).expect(
        format_error_message(&format!("Unable to read file: {}", emojis_json_path)).as_str(),
    );

    let emojis_object: EmojisObject =
        serde_json::from_str(&emojis_json).expect(&format_error_message("Invalid JSON format."));

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
            println!("{}", format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let commit_title: Result<String, InquireError> =
        inquire::Text::new("Enter commit title: ").prompt();

    let commit_title = match commit_title {
        Ok(title) => title,
        Err(e) => {
            println!("{}", format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let commit_message: Result<String, InquireError> =
        inquire::Text::new("Enter commit message: ").prompt();

    let commit_message = match commit_message {
        Ok(message) => message,
        Err(e) => {
            println!("{}", format_error_message(&format!("Error: {}", e)));
            return;
        }
    };

    let mut git_command = format!(
        "git commit -m \"{} {}\"",
        selected_emoji.emoji, commit_title
    );

    if commit_message.len() > 0 {
        git_command = format!("{} -m \"{}\"", git_command, commit_message);
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(git_command)
        .output()
        .expect(&format_error_message("Failed to execute command"));

    if output.status.success() {
        println!(
            "\x1b[0;32mSuccessfully committed with emoji: {}\x1b[0m",
            selected_emoji.emoji
        );
    } else {
        println!(
            "{}",
            format_error_message(&format!(
                "Error: Failed to commit with emoji: {}",
                selected_emoji.emoji
            ))
        );
    }

    println!(
        "Output: {}",
        String::from_utf8_lossy(&output.stdout).to_string()
    );
    println!(
        "Error: {}",
        String::from_utf8_lossy(&output.stderr).to_string()
    );
}
