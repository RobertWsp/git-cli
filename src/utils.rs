pub fn format_error_message(message: &str) -> String {
    return format!("\x1b[0;31m{}\x1b[0m", message);
}

pub fn exit_message(message: &str) {
    eprintln!("{}", format_error_message(message));
    std::process::exit(1);
}
