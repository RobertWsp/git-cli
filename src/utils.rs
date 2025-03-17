pub fn format_error_message(message: &str) -> String {
    return format!("\x1b[0;31m{}\x1b[0m", message);
}
