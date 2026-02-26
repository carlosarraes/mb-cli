use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub metabase: MetabaseConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MetabaseConfig {
    pub url: String,
    pub api_key: Option<String>,
    pub session_token: Option<String>,
}

pub fn config_path() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("could not determine config directory")?
        .join("mb");
    Ok(dir.join("config.toml"))
}

pub fn load() -> Result<Config> {
    let path = config_path()?;
    let content = fs::read_to_string(&path)
        .with_context(|| format!("could not read config at {}\nRun `mb config` to set up", path.display()))?;
    let config: Config = toml::from_str(&content).context("invalid config file")?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("could not create config directory")?;
    }
    let content = toml::to_string_pretty(config).context("could not serialize config")?;
    fs::write(&path, content).context("could not write config file")?;
    Ok(())
}
