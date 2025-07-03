use crate::git::Change;

#[derive(Debug, Clone)]
pub struct CommitTemplate {
    pub name: String,
    pub emoji: String,
    pub title_format: String,
    pub body_template: String,
}

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

    pub fn get_commit_templates() -> Vec<CommitTemplate> {
        vec![
            CommitTemplate {
                name: "Feature".to_string(),
                emoji: "✨".to_string(),
                title_format: "feat: {title}".to_string(),
                body_template: "Add {feature}\n\n- {detail1}\n- {detail2}".to_string(),
            },
            CommitTemplate {
                name: "Bugfix".to_string(),
                emoji: "🐛".to_string(),
                title_format: "fix: {title}".to_string(),
                body_template: "Fix {issue}\n\nResolves #{issue_number}".to_string(),
            },
            CommitTemplate {
                name: "Documentation".to_string(),
                emoji: "📝".to_string(),
                title_format: "docs: {title}".to_string(),
                body_template: "Update documentation for {component}".to_string(),
            },
            CommitTemplate {
                name: "Refactor".to_string(),
                emoji: "♻️".to_string(),
                title_format: "refactor: {title}".to_string(),
                body_template: "Refactor {component}\n\n- Improve {aspect1}\n- Simplify {aspect2}".to_string(),
            },
            CommitTemplate {
                name: "Style".to_string(),
                emoji: "💄".to_string(),
                title_format: "style: {title}".to_string(),
                body_template: "Update styles for {component}".to_string(),
            },
            CommitTemplate {
                name: "Test".to_string(),
                emoji: "✅".to_string(),
                title_format: "test: {title}".to_string(),
                body_template: "Add tests for {component}".to_string(),
            },
            CommitTemplate {
                name: "Chore".to_string(),
                emoji: "🔧".to_string(),
                title_format: "chore: {title}".to_string(),
                body_template: "Update {component}".to_string(),
            },
        ]
    }

    pub fn analyze_changes(changes: &[Change]) -> ChangeAnalysis {
        let mut analysis = ChangeAnalysis::default();
        
        for change in changes {
            match change.change_type.as_str() {
                "Added" => analysis.added_count += 1,
                "Modified" => analysis.modified_count += 1,
                "Deleted" => analysis.deleted_count += 1,
                "Renamed" => analysis.renamed_count += 1,
                "Untracked" => analysis.untracked_count += 1,
                _ => {}
            }
            
            // Analyze file types
            let file_lower = change.value.to_lowercase();
            if file_lower.ends_with(".rs") {
                analysis.rust_files += 1;
            } else if file_lower.ends_with(".js") || file_lower.ends_with(".ts") {
                analysis.js_files += 1;
            } else if file_lower.ends_with(".py") {
                analysis.python_files += 1;
            } else if file_lower.ends_with(".md") {
                analysis.doc_files += 1;
            } else if file_lower.contains("test") {
                analysis.test_files += 1;
            }
        }
        
        analysis
    }
}

#[derive(Debug, Default)]
pub struct ChangeAnalysis {
    pub added_count: usize,
    pub modified_count: usize,
    pub deleted_count: usize,
    pub renamed_count: usize,
    pub untracked_count: usize,
    pub rust_files: usize,
    pub js_files: usize,
    pub python_files: usize,
    pub doc_files: usize,
    pub test_files: usize,
}

impl ChangeAnalysis {
    pub fn suggest_commit_type(&self) -> Option<&'static str> {
        if self.test_files > 0 && self.test_files == self.total_files() {
            Some("test")
        } else if self.doc_files > 0 && self.doc_files == self.total_files() {
            Some("docs")
        } else if self.added_count > 0 && self.modified_count == 0 && self.deleted_count == 0 {
            Some("feat")
        } else if self.deleted_count > self.added_count {
            Some("refactor")
        } else {
            None
        }
    }
    
    pub fn total_files(&self) -> usize {
        self.added_count + self.modified_count + self.deleted_count + self.renamed_count + self.untracked_count
    }
}
