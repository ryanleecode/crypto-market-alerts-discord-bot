use anyhow::{Context, Result};
use config::{Config, File};
use serde::Deserialize;
use std::env;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

#[derive(Debug, Deserialize)]
#[readonly::make]
struct Vault {
    addr: String,
}

#[derive(Debug, Deserialize)]
#[readonly::make]
struct Db {
    username: String,
    hostname: String,
    database_name: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
#[readonly::make]
struct Server {
    port: u16,
}

#[derive(Debug, Deserialize)]
#[readonly::make]
struct InternalSettings {
    debug: bool,
    vault: Vault,
    server: Server,
    db: Db,
}

#[derive(Debug, Deserialize)]
#[readonly::make]
struct Secrets {
    db_pw: String,
    discord_token: String,
}

#[readonly::make]
pub struct Settings {
    pub database_url: String,
    pub discord_token: String,
    pub server_port: u16,
}

impl Settings {
    pub async fn new(vault_token: &str) -> Result<Self> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()
            .with_context(|| "failed to load config")?;

        let internal_settings: InternalSettings = config
            .try_deserialize()
            .with_context(|| "failed to deserialize config")?;
        let vault_client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(&internal_settings.vault.addr)
                .token(vault_token)
                .build()
                .with_context(|| "failed to build vault client settings")?,
        )
        .with_context(|| "failed to create vault client")?;
        let secrets: Secrets = kv2::read(&vault_client, "secret", "cma_bot")
            .await
            .with_context(|| "failed to load secrets from vault")?;

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            internal_settings.db.username,
            secrets.db_pw,
            internal_settings.db.hostname,
            internal_settings.db.port,
            internal_settings.db.database_name
        );

        Ok(Settings {
            database_url,
            discord_token: secrets.discord_token,
            server_port: internal_settings.server.port,
        })
    }
}
