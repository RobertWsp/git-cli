use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::errors::{Result, GitCliError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub commit: CommitConfig,
    pub hooks: HooksConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub default_emoji: String,
    pub auto_push: bool,
    pub confirm_before_push: bool,
    pub debug: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitConfig {
    pub max_title_length: usize,
    pub max_body_length: usize,
    pub auto_capitalize_title: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HooksConfig {
    pub run_pre_commit: bool,
    pub auto_fix_lint: bool,
    pub retry_on_failure: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                default_emoji: "✨".to_string(),
                auto_push: false,
                confirm_before_push: true,
                debug: false,
            },
            commit: CommitConfig {
                max_title_length: 50,
                max_body_length: 72,
                auto_capitalize_title: true,
            },
            hooks: HooksConfig {
                run_pre_commit: true,
                auto_fix_lint: true,
                retry_on_failure: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| GitCliError::ConfigError(format!("Failed to read config: {}", e)))?;
        
        let config: Config = toml::from_str(&config_str)
            .map_err(|e| GitCliError::ConfigError(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config_str = toml::to_string_pretty(self)
            .map_err(|e| GitCliError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(&config_path, config_str)
            .map_err(|e| GitCliError::ConfigError(format!("Failed to write config: {}", e)))?;
        
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| GitCliError::ConfigError("Failed to get home directory".to_string()))?;
        
        Ok(home_dir.join(".config").join("git-cli").join("config.toml"))
    }
}

// Adicionar dependência toml
// No Cargo.toml será necessário adicionar: toml = "0.8"
