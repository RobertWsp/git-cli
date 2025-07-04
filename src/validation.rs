use crate::git::Change;

pub struct ValidationService;

impl ValidationService {
    pub fn suggest_emoji_by_files(changed_files: &[String]) -> Vec<&'static str> {
        let mut suggestions = Vec::new();
        
        for file in changed_files {
            let file_lower = file.to_lowercase();
            
            if file_lower.ends_with(".md") || file_lower.ends_with(".rst") || file_lower.ends_with(".txt") {
                suggestions.push("📝"); // docs
            } else if file_lower.contains("test") || file_lower.contains("spec") {
                suggestions.push("✅"); // test
            } else if file_lower.ends_with(".css") || file_lower.ends_with(".scss") || file_lower.ends_with(".sass") {
                suggestions.push("💄"); // style
            } else if file_lower.ends_with(".json") || file_lower.ends_with(".toml") || file_lower.ends_with(".yaml") || file_lower.ends_with(".yml") {
                suggestions.push("🔧"); // config
            } else if file_lower.starts_with("dockerfile") || file_lower.ends_with(".dockerfile") {
                suggestions.push("🐳"); // docker
            } else if file_lower.contains("security") || file_lower.contains("auth") {
                suggestions.push("🔒"); // security
            } else if file_lower.contains("performance") || file_lower.contains("perf") {
                suggestions.push("⚡"); // performance
            }
        }
        
        // Remove duplicates and return
        suggestions.sort();
        suggestions.dedup();
        suggestions
    }
}
