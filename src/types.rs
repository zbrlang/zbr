use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use sqlx::SqlitePool;

pub type CommandMap = Arc<RwLock<HashMap<String, Command>>>;
pub type Db = Arc<SqlitePool>;

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub trigger: String,
    pub description: String,
    pub command_type: CommandType,
    pub scope: CommandScope,
    pub options: Vec<CommandOption>,
    pub code: String,
}

#[derive(Clone, Debug)]
pub enum CommandType {
    Prefix,
    Slash,
    Interaction,
    Event,
}

#[derive(Clone, Debug)]
pub enum CommandScope {
    Guild,
    Global,
    Both,
}

#[derive(Clone, Debug)]
pub struct CommandOption {
    pub name: String,
    pub description: String,
    pub option_type: OptionType,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub enum OptionType {
    String,
    Integer,
    Number,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Attachment,
}

impl OptionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "string" => Some(OptionType::String),
            "integer" => Some(OptionType::Integer),
            "number" => Some(OptionType::Number),
            "boolean" => Some(OptionType::Boolean),
            "user" => Some(OptionType::User),
            "channel" => Some(OptionType::Channel),
            "role" => Some(OptionType::Role),
            "mentionable" => Some(OptionType::Mentionable),
            "attachment" => Some(OptionType::Attachment),
            _ => None,
        }
    }

    pub fn to_serenity_type(&self) -> serenity::model::application::CommandOptionType {
        use serenity::model::application::CommandOptionType;
        match self {
            OptionType::String => CommandOptionType::String,
            OptionType::Integer => CommandOptionType::Integer,
            OptionType::Number => CommandOptionType::Number,
            OptionType::Boolean => CommandOptionType::Boolean,
            OptionType::User => CommandOptionType::User,
            OptionType::Channel => CommandOptionType::Channel,
            OptionType::Role => CommandOptionType::Role,
            OptionType::Mentionable => CommandOptionType::Mentionable,
            OptionType::Attachment => CommandOptionType::Attachment,
        }
    }
}
