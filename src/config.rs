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

#[cfg(test)]
mod tests {
    use super::*;

    struct TempPath(PathBuf);

    impl Drop for TempPath {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }

    fn temp_config_path(name: &str) -> TempPath {
        TempPath(std::env::temp_dir().join(format!(
            "anyclient-config-test-{}-{name}.toml",
            std::process::id()
        )))
    }

    #[test]
    fn save_and_load_round_trips() {
        let path = temp_config_path("round-trip");

        let config = Config {
            base_url: Some("http://127.0.0.1:31012".into()),
            api_key: Some("secret".into()),
        };
        config.save(&path.0).unwrap();

        let loaded = Config::load(&path.0).unwrap();
        assert_eq!(loaded.base_url.as_deref(), Some("http://127.0.0.1:31012"));
        assert_eq!(loaded.api_key.as_deref(), Some("secret"));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&path.0).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
    }

    #[test]
    fn load_returns_default_for_missing_file() {
        let path = temp_config_path("missing");
        let loaded = Config::load(&path.0).unwrap();
        assert!(loaded.base_url.is_none());
        assert!(loaded.api_key.is_none());
    }

    #[test]
    fn legacy_app_key_alias_still_loads() {
        let config: Config = toml::from_str("app_key = \"legacy\"").unwrap();
        assert_eq!(config.api_key.as_deref(), Some("legacy"));
    }
}
