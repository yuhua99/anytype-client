use std::{fs, path::PathBuf};

#[cfg(unix)]
use std::{
    fs::OpenOptions,
    io::Write,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub base_url: Option<String>,
    #[serde(alias = "app_key")]
    pub api_key: Option<String>,
}

impl Config {
    pub fn path(cli_path: &Option<PathBuf>) -> Result<PathBuf> {
        if let Some(path) = cli_path {
            return Ok(path.clone());
        }
        let home = dirs::home_dir().ok_or_else(|| anyhow!("cannot find home directory"))?;
        Ok(home.join(".anyclient").join("config.toml"))
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        toml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        write_private(path, &toml::to_string_pretty(self)?)
            .with_context(|| format!("failed to write {}", path.display()))
    }
}

#[cfg(unix)]
fn write_private(path: &PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn write_private(path: &PathBuf, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}
