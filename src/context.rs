use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

use std::sync::OnceLock;

pub static START_TIME: OnceLock<std::time::Instant> = OnceLock::new();

#[derive(Clone, Debug, Default)]
pub struct ButtonData {
    pub custom_id: String, // custom_id or URL for link buttons
    pub label: String,
    pub style: String, // primary, secondary, success, danger, link
    pub disabled: bool,
    pub emoji: Option<String>,
    pub new_row: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SelectOptionData {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub default: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SelectMenuData {
    pub menu_id: String,
    pub kind: String, // string, user, role, mentionable, channel
    pub min_values: u8,
    pub max_values: u8,
    pub placeholder: Option<String>,
    pub options: Vec<SelectOptionData>,
}

#[derive(Clone, Debug, Default)]
pub struct ModalFieldData {
    pub field_id: String,
    pub label: String,
    pub style: String, // short or paragraph
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: bool,
    pub placeholder: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ModalData {
    pub modal_id: String,
    pub title: String,
    pub fields: Vec<ModalFieldData>,
}

/// All component state accumulated during a single execution.
#[derive(Clone, Debug, Default)]
pub struct ComponentState {
    /// Buttons queued for the response, in order.
    pub buttons: Vec<ButtonData>,
    /// String select menu queued for the response (at most one per message).
    pub select_menu: Option<SelectMenuData>,
    /// Modal queued to be shown (only valid in interaction responses).
    pub modal: Option<ModalData>,
    /// Whether Zdefer{} was called.
    pub deferred: bool,
}

#[derive(Clone, Default, Debug)]
pub struct Embed {
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
    pub fields: Vec<EmbedField>,
}

impl Embed {
    pub fn has_content(&self) -> bool {
        self.title.is_some()
            || self.title_url.is_some()
            || self.description.is_some()
            || self.color.is_some()
            || self.thumbnail.is_some()
            || self.image.is_some()
            || self.footer.is_some()
            || self.footer_icon.is_some()
            || self.author.is_some()
            || self.author_icon.is_some()
            || self.author_url.is_some()
            || self.timestamp
            || !self.fields.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

/// The live execution context passed to every ZBR function during a command run.
/// This is the single canonical definition — all modules import from here.
#[derive(Clone)]
pub struct DiscordContext {
    pub author_id: String,
    pub username: String,
    pub channel_id: String,
    pub guild_id: String,
    pub message: String,
    pub options: HashMap<String, String>,
    pub bot_id: String,
    pub db: Option<Arc<sqlx::SqlitePool>>,
    /// Indexed embed store. Index is 1-based in ZBR syntax, 0-based here.
    /// Max 10 embeds.
    pub embed: Arc<Mutex<Vec<Embed>>>,
    pub http: Option<Arc<serenity::http::Http>>,
    pub cache: Arc<serenity::cache::Cache>,
    /// Embed indices (0-based) that have been explicitly dispatched via ZsendEmbed.
    /// These are excluded from the automatic current-channel response.
    pub consumed_embeds: Arc<Mutex<HashSet<usize>>>,
    /// Slash command options in declaration order (values only, for positional Zmessage{N} access).
    pub options_list: Vec<String>,
    /// The trigger prefix that was matched (for prefix commands).
    pub trigger: Option<String>,
    /// Active timezone for date/time functions. Defaults to Asia/Tokyo.
    pub timezone: Arc<Mutex<String>>,
    /// Temporary variables scoped to this single execution (Zvar).
    pub temp_vars: Arc<Mutex<HashMap<String, String>>>,
    /// The command name used as the cooldown key (set by the loader/bot).
    pub command_name: String,
    /// Label overrides for cooldown time display (set by ZchangeCooldownTime).
    pub cooldown_labels: Arc<Mutex<CooldownLabels>>,
    /// Reactions to add to the bot's own response after it is sent.
    pub pending_reactions: Arc<Mutex<Vec<String>>>,
    /// The ID of the message that triggered this command (for ZaddCmdReactions).
    pub trigger_message_id: Option<String>,
    /// Temporary split text storage for ZtextSplit / ZsplitText etc.
    pub split_text: Arc<Mutex<Vec<String>>>,
    /// The timestamp when this context/execution began
    pub execution_start: std::time::Instant,
    /// Allowed user mention IDs for the bot's response.
    /// None = Discord default (all mentions allowed).
    /// Some(vec![]) = no user pings allowed.
    /// Some(vec!["id", ...]) = only those user IDs may be pinged.
    pub allowed_user_mentions: Arc<Mutex<Option<Vec<String>>>>,
    /// Allowed role mention IDs for the bot's response.
    /// None = Discord default (all mentions allowed).
    /// Some(vec![]) = no role pings allowed.
    /// Some(vec!["id", ...]) = only those role IDs may be pinged.
    pub allowed_role_mentions: Arc<Mutex<Option<Vec<String>>>>,
    /// If true, the slash command response will be sent as ephemeral (only visible to the invoker).
    pub ephemeral: Arc<Mutex<bool>>,
    /// If set, all bot output for this execution is redirected to this channel ID instead of the
    /// current channel.
    pub use_channel: Arc<Mutex<Option<String>>>,
    /// HTTP headers accumulated by ZhttpAddHeader for the current execution.
    pub http_headers: Arc<Mutex<HashMap<String, String>>>,
    /// Status code of the last HTTP request made in this execution.
    pub http_last_status: Arc<Mutex<u16>>,
    /// Raw response body of the last HTTP request made in this execution.
    pub http_last_body: Arc<Mutex<String>>,
    /// Mutable JSON object for the current execution (ZjsonParse, ZjsonSet, etc.).
    pub json_object: Arc<Mutex<Option<serde_json::Value>>>,
    /// If set, errors are suppressed. Contains the text to show instead (empty = silent).
    pub suppress_error_text: Arc<Mutex<Option<String>>>,
    /// If set, errors are suppressed and this embed index (0-based) is sent instead.
    pub suppress_error_embed: Arc<Mutex<Option<usize>>>,
    /// Component state — buttons, select menus, modals built during this execution.
    pub components: Arc<Mutex<ComponentState>>,
    /// The custom_id of the interaction that triggered this execution (for onInteraction handlers).
    pub custom_id: Option<String>,
    /// Modal field values submitted with this interaction (fieldID → value).
    pub modal_values: HashMap<String, String>,
    /// Values selected in the current component interaction (for select menus).
    pub selected_values: Vec<String>,
    /// Named background tasks spawned by Zasync{}. Awaitable via Zawait{name}.
    pub async_tasks: Arc<Mutex<HashMap<String, tokio::task::JoinHandle<String>>>>,
}

#[derive(Clone, Debug)]
pub struct CooldownLabels {
    pub days: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
}

impl Default for CooldownLabels {
    fn default() -> Self {
        CooldownLabels {
            days: "Days".to_string(),
            hours: "Hours".to_string(),
            minutes: "Minutes".to_string(),
            seconds: "Seconds".to_string(),
        }
    }
}

impl Default for DiscordContext {
    fn default() -> Self {
        DiscordContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: String::new(),
            guild_id: String::new(),
            message: String::new(),
            options: HashMap::new(),
            bot_id: String::new(),
            db: None,
            embed: Arc::new(Mutex::new(vec![])),
            http: None,
            consumed_embeds: Arc::new(Mutex::new(HashSet::new())),
            trigger: None,
            options_list: Vec::new(),
            timezone: Arc::new(Mutex::new("Asia/Tokyo".to_string())),
            temp_vars: Arc::new(Mutex::new(HashMap::new())),
            command_name: String::new(),
            cooldown_labels: Arc::new(Mutex::new(CooldownLabels::default())),
            pending_reactions: Arc::new(Mutex::new(vec![])),
            trigger_message_id: None,
            split_text: Arc::new(Mutex::new(vec![])),
            execution_start: std::time::Instant::now(),
            cache: Arc::new(serenity::cache::Cache::default()),
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
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: Vec::new(),
            async_tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub struct FnMeta {
    pub func: fn(Vec<String>, &DiscordContext) -> FnOutput,
    pub min_args: usize,
    pub max_args: usize,
}

pub enum FnOutput {
    Text(String),
    Reply,
    Empty,
    /// A fatal runtime error shown to the bot developer. Gets "Line N: " prefix.
    Error(String),
    /// A user-facing error message. Shown exactly as-is, no line prefix.
    UserError(String),
}

impl FnOutput {
    pub fn error(function: &str, message: impl Into<String>) -> Self {
        FnOutput::Error(format!("Z{} - {}", function, message.into()))
    }
    pub fn user_error(message: impl Into<String>) -> Self {
        FnOutput::UserError(message.into())
    }
}

pub struct EvalResult {
    pub output: Vec<String>,
    pub should_reply: bool,
    /// At most one error — execution halted on first failure.
    pub errors: Vec<String>,
    pub embeds: Vec<Embed>,
    /// 0-based indices of embeds that were explicitly sent via ZsendEmbed.
    pub consumed_embeds: HashSet<usize>,
    /// If true, slash command response should be ephemeral.
    pub ephemeral: bool,
    /// If set, redirect all output to this channel ID.
    pub use_channel: Option<String>,
    /// Component state to attach to the response.
    pub components: ComponentState,
}
