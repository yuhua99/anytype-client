pub mod api;
pub mod cli;
pub(crate) mod commands;
pub(crate) mod config;
pub mod models;
pub mod output;
pub(crate) mod services;

use anyhow::{Result, anyhow};
use clap::Parser;

use crate::{
    api::AnytypeClient,
    cli::{Cli, Command},
    commands::{auth_command, run_command},
    config::Config,
};

pub const DEFAULT_BASE_URL: &str = DEFAULT_HEADLESS_BASE_URL;
pub const DEFAULT_DESKTOP_BASE_URL: &str = "http://127.0.0.1:31009";
pub const DEFAULT_HEADLESS_BASE_URL: &str = "http://127.0.0.1:31012";
pub const ANYTYPE_VERSION: &str = "2025-11-08";

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let config_path = Config::path(&cli.config)?;
    let mut config = Config::load(&config_path)?;

    if let Some(base_url) = cli.base_url.clone() {
        config.base_url = Some(base_url);
    }

    let cli_api_key = cli.api_key.clone();
    match cli.command {
        Command::Auth(args) => {
            auth_command(args, config, config_path, cli.base_url, cli_api_key).await
        }
        command => {
            if let Some(api_key) = cli_api_key {
                config.api_key = Some(api_key);
            }
            let base_url = config
                .base_url
                .clone()
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
            let api_key = config.api_key.clone().ok_or_else(|| anyhow!("not authenticated: run `anyclient auth headless --api-key <key>` or `anyclient auth desktop`, or set ANYTYPE_API_KEY"))?;
            let client = AnytypeClient::new(base_url, Some(api_key))?;
            run_command(command, client, cli.output).await
        }
    }
}
