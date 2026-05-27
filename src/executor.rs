use crate::context::{CooldownLabels, ComponentState, DiscordContext};
use crate::runtime;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

// ── Output types (consumed by bot.rs response processing) ───────────────

pub struct EmbedFieldData {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

pub struct EmbedData {
    pub title: Option<String>,
    pub title_url: Option<String>,
    pub description: Option<String>,
    pub color: Option<u32>,
    pub thumbnail: Option<String>,
    pub image: Option<String>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub author: Option<String>,
    pub author_icon: Option<String>,
    pub author_url: Option<String>,
    pub timestamp: bool,
    pub fields: Vec<EmbedFieldData>,
}

pub struct ButtonData {
    pub custom_id: String,
    pub label: String,
    pub style: String,
    pub disabled: bool,
    pub emoji: Option<String>,
    pub new_row: bool,
}

pub struct SelectOptionData {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub default: bool,
}

pub struct SelectMenuData {
    pub menu_id: String,
    pub kind: String,
    pub min_values: u8,
    pub max_values: u8,
    pub placeholder: Option<String>,
    pub options: Vec<SelectOptionData>,
}

pub struct ModalFieldData {
    pub field_id: String,
    pub label: String,
    pub style: String,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: bool,
    pub placeholder: Option<String>,
    pub value: Option<String>,
}

pub struct ModalData {
    pub modal_id: String,
    pub title: String,
    pub fields: Vec<ModalFieldData>,
}

pub struct ComponentData {
    pub buttons: Vec<ButtonData>,
    pub select_menu: Option<SelectMenuData>,
    pub modal: Option<ModalData>,
    pub deferred: bool,
    pub update_message: bool,
}

pub struct RunResponse {
    pub output: Vec<String>,
    pub errors: Vec<String>,
    pub should_reply: bool,
    pub embeds: Vec<EmbedData>,
    pub pending_reactions: Vec<String>,
    pub allowed_user_mentions: Option<Vec<String>>,
    pub allowed_role_mentions: Option<Vec<String>>,
    pub ephemeral: bool,
    pub use_channel: Option<String>,
    pub components: ComponentData,
}

// ── Input types ─────────────────────────────────────────────────────────

pub struct RunContext {
    pub author_id: String,
    pub username: String,
    pub channel_id: String,
    pub guild_id: String,
    pub message: String,
    pub options: HashMap<String, String>,
    pub options_list: Vec<String>,
    pub trigger: Option<String>,
    pub command_name: String,
    pub trigger_message_id: Option<String>,
    pub custom_id: Option<String>,
    pub modal_values: HashMap<String, String>,
    pub selected_values: Vec<String>,
}

pub struct ExecState {
    pub db: crate::types::Db,
    pub bot_id: String,
    pub http: Option<Arc<serenity::http::Http>>,
    pub cache: Arc<serenity::cache::Cache>,
}

// ── DiscordContext builder ──────────────────────────────────────────────

fn build_discord_context(ctx: RunContext, state: &ExecState) -> DiscordContext {
    DiscordContext {
        author_id: ctx.author_id,
        username: ctx.username,
        channel_id: ctx.channel_id,
        guild_id: ctx.guild_id,
        message: ctx.message,
        options: ctx.options,
        options_list: ctx.options_list,
        bot_id: state.bot_id.clone(),
        db: Some(state.db.clone()),
        embed: Arc::new(Mutex::new(vec![])),
        http: state.http.clone(),
        consumed_embeds: Arc::new(Mutex::new(HashSet::new())),
        trigger: ctx.trigger,
        timezone: Arc::new(Mutex::new("Asia/Tokyo".to_string())),
        temp_vars: Arc::new(Mutex::new(HashMap::new())),
        command_name: ctx.command_name,
        cooldown_labels: Arc::new(Mutex::new(CooldownLabels::default())),
        pending_reactions: Arc::new(Mutex::new(vec![])),
        trigger_message_id: ctx.trigger_message_id,
        split_text: Arc::new(Mutex::new(vec![])),
        execution_start: std::time::Instant::now(),
        cache: state.cache.clone(),
        allowed_user_mentions: Arc::new(Mutex::new(None)),
        allowed_role_mentions: Arc::new(Mutex::new(None)),
        ephemeral: Arc::new(Mutex::new(false)),
        use_channel: Arc::new(Mutex::new(None)),
        http_headers: Arc::new(Mutex::new(HashMap::new())),
        http_last_status: Arc::new(Mutex::new(0)),
        http_last_body: Arc::new(Mutex::new(String::new())),
        json_object: Arc::new(Mutex::new(None)),
        suppress_error_text: Arc::new(Mutex::new(None)),
        suppress_error_embed: Arc::new(Mutex::new(None)),
        components: Arc::new(Mutex::new(ComponentState::default())),
        custom_id: ctx.custom_id,
        modal_values: ctx.modal_values,
        selected_values: ctx.selected_values,
        async_tasks: Arc::new(Mutex::new(HashMap::new())),
    }
}

// ── Response builder ────────────────────────────────────────────────────

fn build_response(result: crate::context::EvalResult, rt: &mut runtime::Runtime) -> RunResponse {
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

    let mut embeds: Vec<EmbedData> = Vec::new();
    for (i, e) in result.embeds.into_iter().enumerate() {
        if !e.has_content() || result.consumed_embeds.contains(&i) {
            continue;
        }
        let has_required = e.title.is_some()
            || e.description.is_some()
            || e.author.is_some()
            || !e.fields.is_empty();
        if !has_required {
            return RunResponse {
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
                components: ComponentData {
                    buttons: vec![],
                    select_menu: None,
                    modal: None,
                    deferred: false,
                    update_message: false,
                },
            };
        }
        embeds.push(EmbedData {
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

    let components = ComponentData {
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
        update_message: result.components.update_message,
    };

    RunResponse {
        output: result.output,
        errors: result.errors,
        should_reply: result.should_reply,
        embeds,
        pending_reactions,
        allowed_user_mentions,
        allowed_role_mentions,
        ephemeral: result.ephemeral,
        use_channel: result.use_channel,
        components,
    }
}

// ── Public entry point ──────────────────────────────────────────────────

pub fn execute_code(code: &str, ctx: RunContext, state: &ExecState) -> RunResponse {
    let discord_ctx = build_discord_context(ctx, state);
    let mut rt = runtime::Runtime::new(discord_ctx);
    let result = rt.run(code);
    build_response(result, &mut rt)
}
