mod ast;
mod bot;
mod context;
mod db;
mod functions;
mod loader;
mod parser;
mod runtime;
mod types;

use axum::{extract::State, routing::post, Json, Router};
use bot::Bot;
use context::DiscordContext;
use dotenv::dotenv;
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serenity::prelude::*;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use types::{CommandMap, Db};

#[derive(Clone)]
struct AppState {
    commands: CommandMap,
    db: Db,
    bot_id: String,
    http: Option<Arc<serenity::http::Http>>,
    cache: Arc<serenity::cache::Cache>,
}

#[derive(Deserialize)]
struct RunRequest {
    code: String,
    context: Option<BotContext>,
}

#[derive(Deserialize, Clone)]
pub struct BotContext {
    pub author_id: String,
    pub username: String,
    pub channel_id: String,
    pub guild_id: String,
    pub message: String,
    pub options: Option<HashMap<String, String>>,
    pub options_list: Option<Vec<String>>,
    pub trigger: Option<String>,
    pub command_name: Option<String>,
    pub trigger_message_id: Option<String>,
    pub custom_id: Option<String>,
    pub modal_values: Option<HashMap<String, String>>,
    pub selected_values: Option<Vec<String>>,
}

#[derive(Serialize)]
struct RunResponse {
    output: Vec<String>,
    errors: Vec<String>,
    should_reply: bool,
    embeds: Vec<EmbedData>,
    pending_reactions: Vec<String>,
    allowed_user_mentions: Option<Vec<String>>,
    allowed_role_mentions: Option<Vec<String>>,
    ephemeral: bool,
    use_channel: Option<String>,
    components: ComponentData,
}

#[derive(Serialize, Default)]
struct ComponentData {
    buttons: Vec<ButtonData>,
    select_menu: Option<SelectMenuData>,
    modal: Option<ModalData>,
    deferred: bool,
}

#[derive(Serialize)]
struct ButtonData {
    custom_id: String,
    label: String,
    style: String,
    disabled: bool,
    emoji: Option<String>,
    new_row: bool,
}

#[derive(Serialize)]
struct SelectMenuData {
    menu_id: String,
    kind: String,
    min_values: u8,
    max_values: u8,
    placeholder: Option<String>,
    options: Vec<SelectOptionData>,
}

#[derive(Serialize)]
struct SelectOptionData {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<String>,
    default: bool,
}

#[derive(Serialize)]
struct ModalData {
    modal_id: String,
    title: String,
    fields: Vec<ModalFieldData>,
}

#[derive(Serialize)]
struct ModalFieldData {
    field_id: String,
    label: String,
    style: String,
    min_length: Option<u32>,
    max_length: Option<u32>,
    required: bool,
    placeholder: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Clone)]
struct EmbedData {
    title: Option<String>,
    title_url: Option<String>,
    description: Option<String>,
    color: Option<u32>,
    thumbnail: Option<String>,
    image: Option<String>,
    footer: Option<String>,
    footer_icon: Option<String>,
    author: Option<String>,
    author_icon: Option<String>,
    author_url: Option<String>,
    timestamp: bool,
    fields: Vec<EmbedFieldData>,
}

#[derive(Serialize, Clone)]
struct EmbedFieldData {
    name: String,
    value: String,
    inline: bool,
}

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

    // Share the HTTP client with the axum server so ZsendEmbed can use it
    let http = client.http.clone();

    let state = AppState {
        commands,
        db: database,
        bot_id,
        http: Some(http),
        cache: client.cache.clone(),
    };

    let app = Router::new()
        .route("/run", post(run_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("ZBR server running on http://localhost:3000");

    tokio::select! {
        _ = axum::serve(listener, app) => {},
        _ = client.start() => {},
    }

    drop(watcher);
}

async fn run_handler(
    State(state): State<AppState>,
    Json(payload): Json<RunRequest>,
) -> Json<RunResponse> {
    let ctx = payload
        .context
        .map(|c| DiscordContext {
            author_id: c.author_id,
            username: c.username,
            channel_id: c.channel_id,
            guild_id: c.guild_id,
            message: c.message,
            options: c.options.unwrap_or_default(),
            options_list: c.options_list.unwrap_or_default(),
            bot_id: state.bot_id.clone(),
            db: Some(state.db.clone()),
            embed: Arc::new(tokio::sync::Mutex::new(vec![])),
            http: state.http.clone(),
            consumed_embeds: Arc::new(tokio::sync::Mutex::new(std::collections::HashSet::new())),
            trigger: c.trigger,
            timezone: Arc::new(tokio::sync::Mutex::new("Asia/Tokyo".to_string())),
            temp_vars: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            command_name: c.command_name.unwrap_or_default(),
            cooldown_labels: Arc::new(tokio::sync::Mutex::new(context::CooldownLabels::default())),
            pending_reactions: Arc::new(tokio::sync::Mutex::new(vec![])),
            trigger_message_id: c.trigger_message_id,
            split_text: Arc::new(tokio::sync::Mutex::new(vec![])),
            execution_start: std::time::Instant::now(),
            cache: state.cache.clone(),
            allowed_user_mentions: Arc::new(tokio::sync::Mutex::new(None)),
            allowed_role_mentions: Arc::new(tokio::sync::Mutex::new(None)),
            ephemeral: Arc::new(tokio::sync::Mutex::new(false)),
            use_channel: Arc::new(tokio::sync::Mutex::new(None)),
            http_headers: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            http_last_status: Arc::new(tokio::sync::Mutex::new(0)),
            http_last_body: Arc::new(tokio::sync::Mutex::new(String::new())),
            json_object: Arc::new(tokio::sync::Mutex::new(None)),
            suppress_error_text: Arc::new(tokio::sync::Mutex::new(None)),
            suppress_error_embed: Arc::new(tokio::sync::Mutex::new(None)),
            components: Arc::new(tokio::sync::Mutex::new(context::ComponentState::default())),
            custom_id: c.custom_id,
            modal_values: c.modal_values.unwrap_or_default(),
            selected_values: c.selected_values.unwrap_or_default(),
            async_tasks: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
        })
        .unwrap_or_default();

    let mut rt = runtime::Runtime::new(ctx);
    let result = rt.run(&payload.code);

    let embeds: Vec<EmbedData> = {
        let mut out = Vec::new();
        for (i, e) in result.embeds.into_iter().enumerate() {
            if !e.has_content() || result.consumed_embeds.contains(&i) {
                continue;
            }
            // Validate Discord requirements — turn invalid embeds into errors
            let has_required = e.title.is_some()
                || e.description.is_some()
                || e.author.is_some()
                || !e.fields.is_empty();
            if !has_required {
                // Return an error response instead of sending a bad embed
                return Json(RunResponse {
                    output: vec![],
                    errors: vec![format!(
                        "embed {} must have at least a title, description, author, or field before it can be sent",
                        i + 1
                    )],
                    should_reply: false,
                    embeds: vec![],
                    pending_reactions: vec![],
                    allowed_user_mentions: None,
                    allowed_role_mentions: None,
                    ephemeral: false,
                    use_channel: None,
                    components: ComponentData::default(),
                });
            }
            out.push(EmbedData {
                title: e.title,
                title_url: e.title_url,
                description: e.description,
                color: e.color,
                thumbnail: e.thumbnail,
                image: e.image,
                footer: e.footer,
                footer_icon: e.footer_icon,
                author: e.author,
                author_icon: e.author_icon,
                author_url: e.author_url,
                timestamp: e.timestamp,
                fields: e
                    .fields
                    .into_iter()
                    .map(|f| EmbedFieldData {
                        name: f.name,
                        value: f.value,
                        inline: f.inline,
                    })
                    .collect(),
            });
        }
        out
    };

    let pending_reactions = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { rt.context.pending_reactions.lock().await.clone() })
    });

    let allowed_user_mentions = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { rt.context.allowed_user_mentions.lock().await.clone() })
    });

    let allowed_role_mentions = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { rt.context.allowed_role_mentions.lock().await.clone() })
    });

    Json(RunResponse {
        output: result.output,
        errors: result.errors,
        should_reply: result.should_reply,
        embeds,
        pending_reactions,
        allowed_user_mentions,
        allowed_role_mentions,
        ephemeral: result.ephemeral,
        use_channel: result.use_channel,
        components: ComponentData {
            buttons: result
                .components
                .buttons
                .into_iter()
                .map(|b| ButtonData {
                    custom_id: b.custom_id,
                    label: b.label,
                    style: b.style,
                    disabled: b.disabled,
                    emoji: b.emoji,
                    new_row: b.new_row,
                })
                .collect(),
            select_menu: result.components.select_menu.map(|s| SelectMenuData {
                menu_id: s.menu_id,
                kind: s.kind,
                min_values: s.min_values,
                max_values: s.max_values,
                placeholder: s.placeholder,
                options: s
                    .options
                    .into_iter()
                    .map(|o| SelectOptionData {
                        label: o.label,
                        value: o.value,
                        description: o.description,
                        emoji: o.emoji,
                        default: o.default,
                    })
                    .collect(),
            }),
            modal: result.components.modal.map(|m| ModalData {
                modal_id: m.modal_id,
                title: m.title,
                fields: m
                    .fields
                    .into_iter()
                    .map(|f| ModalFieldData {
                        field_id: f.field_id,
                        label: f.label,
                        style: f.style,
                        min_length: f.min_length,
                        max_length: f.max_length,
                        required: f.required,
                        placeholder: f.placeholder,
                        value: f.value,
                    })
                    .collect(),
            }),
            deferred: result.components.deferred,
        },
    })
}
