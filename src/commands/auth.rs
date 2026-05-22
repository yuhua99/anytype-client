use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::{
    DEFAULT_DESKTOP_BASE_URL, DEFAULT_HEADLESS_BASE_URL,
    api::AnytypeClient,
    cli::{AuthArgs, AuthCommand},
    config::Config,
};

pub async fn auth_command(
    args: AuthArgs,
    config: Config,
    config_path: PathBuf,
    cli_base_url: Option<String>,
    cli_api_key: Option<String>,
) -> Result<()> {
    match args.command {
        AuthCommand::Desktop { app_name, force } => {
            desktop_auth(config, config_path, cli_base_url, app_name, force).await
        }
        AuthCommand::Headless { api_key, force } => headless_auth(
            config,
            config_path,
            cli_base_url,
            cli_api_key,
            api_key,
            force,
        ),
    }
}

async fn desktop_auth(
    mut config: Config,
    config_path: PathBuf,
    cli_base_url: Option<String>,
    app_name: String,
    force: bool,
) -> Result<()> {
    if config.api_key.is_some() && !force {
        println!("Already authenticated. Use --force to re-authenticate.");
        return Ok(());
    }

    let base_url = cli_base_url.unwrap_or_else(|| DEFAULT_DESKTOP_BASE_URL.to_string());
    let client = AnytypeClient::new(base_url.clone(), None)?;
    let challenge = client.create_challenge(&app_name).await?;

    println!("Desktop auth challenge created. Check Anytype desktop app.");
    println!("Challenge ID: {}", challenge.challenge_id);
    println!("Enter verification code:");

    let mut code = String::new();
    std::io::stdin().read_line(&mut code)?;
    let token = client
        .create_api_key(&challenge.challenge_id, code.trim())
        .await?;

    config.base_url = Some(base_url);
    config.api_key = Some(token.api_key);
    config.save(&config_path)?;
    println!(
        "Desktop authentication successful. Saved credentials to {}",
        config_path.display()
    );
    Ok(())
}

fn headless_auth(
    mut config: Config,
    config_path: PathBuf,
    cli_base_url: Option<String>,
    cli_api_key: Option<String>,
    api_key: Option<String>,
    force: bool,
) -> Result<()> {
    let new_key = api_key.or(cli_api_key);
    if config.api_key.is_some() && !force && new_key.is_none() {
        println!("Already authenticated. Use --force to overwrite.");
        return Ok(());
    }

    let key = new_key.ok_or_else(|| {
        anyhow!("missing API key: run `anytype auth apikey create <name>`, then `anyclient auth headless --api-key <key>`")
    })?;

    config.base_url = Some(cli_base_url.unwrap_or_else(|| DEFAULT_HEADLESS_BASE_URL.to_string()));
    config.api_key = Some(key);
    config.save(&config_path)?;
    println!("Headless API key saved to {}", config_path.display());
    Ok(())
}
