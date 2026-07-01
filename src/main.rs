mod ast;
mod bot;
mod context;
mod db;
mod error_messages;
mod executor;
mod functions;
mod loader;
mod parser;
mod runtime;
mod types;

use bot::Bot;
use dotenv::dotenv;
use notify::{ recommended_watcher, Event, RecursiveMode, Watcher };
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::RwLock;
use types::CommandMap;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--version".to_string()) {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    dotenv().ok();

    if args.contains(&"--validate".to_string()) {
        println!("Validating commands...");
        let mut errors = false;
        if env::var("DISCORD_TOKEN").is_err() {
            println!("Error: Missing DISCORD_TOKEN");
            errors = true;
        }
        if env::var("DATABASE_URL").is_err() {
            println!("Error: Missing DATABASE_URL");
            errors = true;
        }

        let mut registry = std::collections::HashMap::new();
        functions::register(&mut registry);

        // This will print errors if loading fails
        loader::load_commands("commands", &registry);

        println!("Validation complete.");
        if errors {
            std::process::exit(1);
        }
        return;
    }

    crate::context::START_TIME.set(std::time::Instant::now()).unwrap();
    let database = Arc::new(db::connect().await);
    let bot_id = env::var("BOT_ID").unwrap_or_else(|_| "default".to_string());
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env");
    let guild_id = env
        ::var("GUILD_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok());

    let mut registry = std::collections::HashMap::new();
    functions::register(&mut registry);
    let commands: CommandMap = Arc::new(RwLock::new(loader::load_commands("commands", &registry)));

    let commands_for_watcher = commands.clone();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(10);
    let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
        if res.is_ok() {
            let _ = tx.blocking_send(());
        }
    }).unwrap();
    watcher.watch(std::path::Path::new("commands"), RecursiveMode::Recursive).unwrap();

    let watcher_registry = registry.clone();
    tokio::spawn(async move {
        while let Some(_) = rx.recv().await {
            // Debounce: Wait for events to stop coming in for 300ms
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(300)) => {
                        // Timer expired, perform reload
                        break;
                    }
                    _ = rx.recv() => {
                        // Another event came in, reset timer
                        continue;
                    }
                }
            }

            println!("Commands folder changed, reloading...");
            let new_commands = loader::load_commands("commands", &watcher_registry);
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

    let intents =
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::GUILD_PRESENCES |
        GatewayIntents::GUILD_MESSAGE_REACTIONS |
        GatewayIntents::GUILD_MEMBERS |
        GatewayIntents::GUILD_MODERATION |
        GatewayIntents::GUILDS |
        GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(bot).await
        .expect("Failed to create Discord client");

    {
        let mut data = client.data.write().await;
        data.insert::<types::ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    let client_handle = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            eprintln!("Client error: {:?}", why);
        }
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("\nShutting down...");
            shard_manager.shutdown_all().await;
        }
        _ = client_handle => {}
    }

    drop(watcher);
}
