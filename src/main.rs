mod ast;
mod bot;
mod context;
mod db;
mod executor;
mod functions;
mod loader;
mod parser;
mod runtime;
mod types;

use bot::Bot;
use dotenv::dotenv;
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use types::CommandMap;

#[tokio::main]
async fn main() {
    dotenv().ok();
    crate::context::START_TIME
        .set(std::time::Instant::now())
        .unwrap();
    let database = Arc::new(db::connect().await);
    let bot_id = env::var("BOT_ID").unwrap_or_else(|_| "default".to_string());
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env");
    let guild_id = env::var("GUILD_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok());

    let commands: CommandMap = Arc::new(RwLock::new(loader::load_commands("commands")));

    let commands_for_watcher = commands.clone();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(10);
    let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
        if res.is_ok() {
            let _ = tx.blocking_send(());
        }
    })
    .unwrap();
    watcher
        .watch(std::path::Path::new("commands"), RecursiveMode::Recursive)
        .unwrap();

    tokio::spawn(async move {
        while rx.recv().await.is_some() {
            println!("Commands folder changed, reloading...");
            let new_commands = loader::load_commands("commands");
            let mut map = commands_for_watcher.write().await;
            *map = new_commands;
            println!("Commands reloaded");
        }
    });

    let bot = Bot {
        commands: commands.clone(),
        guild_id,
        db: database.clone(),
        bot_id: bot_id.clone(),
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Failed to create Discord client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }

    drop(watcher);
}
