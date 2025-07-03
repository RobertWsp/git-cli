use log::info;

pub fn format_error_message(message: &str) -> String {
    format!("\x1b[0;31m{}\x1b[0m", message)
}

pub fn format_success_message(message: &str) -> String {
    format!("\x1b[0;32m{}\x1b[0m", message)
}

pub fn format_warning_message(message: &str) -> String {
    format!("\x1b[0;33m{}\x1b[0m", message)
}

pub fn format_info_message(message: &str) -> String {
    format!("\x1b[0;36m{}\x1b[0m", message)
}

pub fn init_logger(debug: bool) {
    let log_level = if debug { "debug" } else { "info" };
    
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(log_level)
    )
    .format_timestamp(None)
    .format_module_path(false)
    .format_target(false)
    .init();
    
    if debug {
        info!("Debug logging enabled");
    }
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}
