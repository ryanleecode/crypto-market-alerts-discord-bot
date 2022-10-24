use anyhow::{Context, Result};
use sea_orm::{Database, DatabaseConnection};
use serenity::framework::standard::StandardFramework;
use serenity::prelude::*;

use tokio::sync::broadcast;
use tracing::error;
use tracing::info;

mod bot;
mod cli;
mod db;
mod error_handling;
mod settings;
mod webserver;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let cli_args = cli::CliArgs::get_cli_args("CMABot");
    let settings = settings::Settings::new(&cli_args.vault_token).await?;

    let (shutdown_send, mut shutdown_receive) = broadcast::channel::<()>(1);

    let db = setup_db(&settings.database_url).await?;

    let routes = webserver::setup_routes(db.clone());
    let error_handler = error_handling::BotErrorHandler::new(shutdown_send.clone());

    let mut discord_client = setup_discord_bot(
        &settings.discord_token,
        cli_args.guild_id,
        db,
        error_handler,
    )
    .await?;
    let shard_manager = discord_client.shard_manager.clone();

    let mut webserver_shutdown_recv = shutdown_send.subscribe();
    let (_, webserver) = warp::serve(routes).bind_with_graceful_shutdown(
        ([127, 0, 0, 1], settings.server_port),
        async move {
            webserver_shutdown_recv.recv().await.expect("shutdown");
            info!("webserver shutting down");
        },
    );

    let shutdown_1 = shutdown_send.clone();
    let discord_task = async move {
        if let Err(err) = discord_client.start().await {
            error!("failed to start discord client: {}", err);
            shutdown_1.send(()).expect("shutdown");
        }
    };

    let shutdown_2 = shutdown_send.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("ctrl+c listener");
        info!("ctrl+c signal received");
        shutdown_2.send(()).expect("shutdown");
    });

    tokio::spawn(async move {
        shutdown_receive.recv().await.expect("shutdown");
        info!("shutdown signal received");
        tokio::spawn(async move {
            // needs to run in a loop incase shutdown signal is received before
            // the first shard is spawned.
            loop {
                shard_manager.lock().await.shutdown_all().await;
            }
        });
    });

    // wait for webserver and discord bot to cleanly shutdown
    tokio::join!(webserver, discord_task);

    Ok(())
}

async fn setup_db(database_url: &str) -> Result<DatabaseConnection> {
    Database::connect(database_url)
        .await
        .with_context(|| "failed to connect to database")
}

async fn setup_discord_bot(
    discord_token: &str,
    guild_id: Option<u64>,
    db: DatabaseConnection,
    error_handler: error_handling::BotErrorHandler,
) -> Result<serenity::Client> {
    let framework = StandardFramework::new();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    serenity::Client::builder(&discord_token, intents)
        .event_handler(bot::handler::Handler::new(db, guild_id, error_handler))
        .framework(framework)
        .await
        .with_context(|| "failed to create discord client")
}
