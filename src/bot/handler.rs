use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use anyhow::Context;
use anyhow::Result;
use sea_orm::DatabaseConnection;
use serenity::builder::CreateEmbed;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use tracing::info;

use crate::db;

#[async_trait::async_trait]
pub trait ErrorHandler: Send + Sync {
    async fn on_error(&self, e: HandlerError) {}
}

pub struct Handler<T: ErrorHandler> {
    db: DatabaseConnection,
    /// Guild id for testing purposes
    ///
    /// Global commands take 1 hour to register on discord, so for testing
    /// purpose assign your test server's guild id so the commands work
    /// instantly.
    guild_id: Option<GuildId>,
    error_handler: T,
}

impl<T: ErrorHandler> Handler<T> {
    pub fn new(db: DatabaseConnection, guild_id: Option<u64>, error_handler: T) -> Handler<T> {
        Handler {
            db,
            guild_id: guild_id.map(GuildId),
            error_handler,
        }
    }
}

pub enum HandlerError {
    Setup(anyhow::Error),
    Interaction(anyhow::Error),
    Internal(anyhow::Error),
}

async fn ready<T: ErrorHandler>(
    handler: &Handler<T>,
    ctx: serenity::client::Context,
    ready: Ready,
) -> Result<(), HandlerError> {
    info!("{} is connected!", ready.user.name);

    if let Some(guild_id) = &handler.guild_id {
        GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| super::command::alert::register(command))
        })
        .await
        .with_context(|| format!("failed to set application commands in guild: {}", &guild_id))
        .map_err(HandlerError::Setup)?;
    }

    Command::set_global_application_commands(&ctx.http, |commands| {
        commands.create_application_command(|command| super::command::alert::register(command))
    })
    .await
    .with_context(|| "failed to set global application commands")
    .map_err(HandlerError::Setup)?;

    Ok(())
}

async fn interaction_create<T: ErrorHandler>(
    handler: &Handler<T>,
    ctx: serenity::client::Context,
    interaction: Interaction,
) -> Result<(), HandlerError> {
    if let Interaction::ApplicationCommand(command) = interaction {
        info!("received command interaction: {}", command.data.name);

        if command.data.name == "alerts" {
            let alerts = super::command::alert::run(&handler.db, &command)
                .await
                .with_context(|| "failed to get alerts")
                .map_err(HandlerError::Internal)?;

            let mut tickers_by_interval: HashMap<String, HashSet<String>> = HashMap::new();
            for alert in alerts {
                let tickers = tickers_by_interval
                    .entry(alert.interval)
                    .or_insert(HashSet::new());
                tickers.insert(alert.ticker);
            }

            let mut fields = Vec::new();
            for interval in tickers_by_interval.keys() {
                fields.push((
                    interval,
                    tickers_by_interval
                        .get(interval)
                        .unwrap_or(&HashSet::new())
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect::<Vec<_>>()
                        .join(","),
                    true,
                ))
            }

            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content("alerts found")
                                .add_embed(CreateEmbed(HashMap::new()).fields(fields).to_owned())
                        })
                })
                .await
                .with_context(|| "failed to send interaction alert response")
                .map_err(HandlerError::Interaction)?;
        }
    }

    Ok(())
}

#[serenity::async_trait]
impl<T: ErrorHandler> EventHandler for Handler<T> {
    async fn ready(&self, ctx: serenity::client::Context, r: Ready) {
        if let Err(err) = ready(self, ctx, r).await {
            self.error_handler.on_error(err).await;
        }
    }

    async fn interaction_create(&self, ctx: serenity::client::Context, interaction: Interaction) {
        if let Err(err) = interaction_create(self, ctx, interaction).await {
            self.error_handler.on_error(err).await;
        }
    }
}
