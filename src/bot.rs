use crate::executor::{ self, ComponentData, EmbedData, ExecState, RunContext, RunResponse };
use crate::types::{ Command, CommandMap, CommandScope, CommandType, Db };
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::async_trait;
use serenity::builder::{
    CreateActionRow,
    CreateAllowedMentions,
    CreateButton,
    CreateCommand,
    CreateCommandOption,
    CreateEmbed,
    CreateEmbedAuthor,
    CreateEmbedFooter,
    CreateInputText,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    CreateMessage,
    CreateModal,
    CreateSelectMenu,
    CreateSelectMenuKind,
    CreateSelectMenuOption,
};
use serenity::gateway::ActivityData;
use serenity::model::application::{ CommandOptionType, Interaction };
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::{ EmojiId, GuildId };
use serenity::model::prelude::{
    Guild,
    GuildChannel,
    GuildMemberUpdateEvent,
    Message,
    MessageUpdateEvent,
    PartialGuild,
    Reaction,
    ReactionType,
    Role,
    UnavailableGuild,
    User,
    VoiceState,
};
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;
use std::collections::HashMap;

static VAR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Z(getVar|getServerVar|getUserVar|getChannelVar)\{([^}]+)\}").unwrap()
});

async fn resolve_trigger_vars(
    trigger: &str,
    db: &crate::types::Db,
    bot_id: &str,
    msg: &Message,
    cache: &mut HashMap<String, String>
) -> String {
    let mut result = trigger.to_string();

    // Limit to 5 iterations to prevent infinite loops if values contain variable syntax
    let mut iterations = 0;
    while let Some(caps) = VAR_REGEX.captures(&result.clone()) {
        if iterations >= 5 {
            break;
        }
        iterations += 1;

        let full_match = caps.get(0).unwrap().as_str();
        let func = caps.get(1).unwrap().as_str();
        let var_name = caps.get(2).unwrap().as_str();

        let cache_key = format!("{}:{}", func, var_name);
        let value = if let Some(val) = cache.get(&cache_key) {
            val.clone()
        } else {
            let val = match func {
                "getVar" => crate::db::get_global_var(db, bot_id, var_name).await,
                "getServerVar" => {
                    let guild_id = msg.guild_id.map(|g| g.to_string()).unwrap_or_default();
                    crate::db::get_server_var(db, bot_id, &guild_id, var_name).await
                }
                "getUserVar" => {
                    let guild_id = msg.guild_id.map(|g| g.to_string()).unwrap_or_default();
                    crate::db::get_user_var(
                        db,
                        bot_id,
                        &guild_id,
                        &msg.author.id.to_string(),
                        var_name
                    ).await
                }
                "getChannelVar" => {
                    crate::db::get_channel_var(
                        db,
                        bot_id,
                        &msg.channel_id.to_string(),
                        var_name
                    ).await
                }
                _ => String::new(),
            };
            cache.insert(cache_key, val.clone());
            val
        };

        result = result.replace(full_match, &value);
    }

    result
}

async fn get_latency(ctx: &Context) -> Option<std::time::Duration> {
    let data = ctx.data.read().await;
    let shard_manager = data.get::<crate::types::ShardManagerContainer>()?;
    let runners = shard_manager.runners.lock().await;
    runners.get(&ctx.shard_id)?.latency
}

async fn total_shard_count(ctx: &Context) -> u64 {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<crate::types::ShardManagerContainer>() {
        Some(sm) => sm,
        None => {
            return 1;
        }
    };
    let runners = shard_manager.runners.lock().await;
    runners.len() as u64
}

impl Bot {
    async fn run_event_command(&self, ctx: &Context, trigger: &str, context: RunContext) {
        let commands = self.commands.read().await;
        if
            let Some(cmd) = commands
                .get(trigger)
                .filter(|c| matches!(c.command_type, CommandType::Event))
        {
            let ast = cmd.ast.clone();
            let channel_id = context.channel_id.clone();
            drop(commands);
            let state = ExecState {
                db: self.db.clone(),
                bot_id: self.bot_id.clone(),
                http: Some(ctx.http.clone()),
                cache: ctx.cache.clone(),
                shard_latency: get_latency(&ctx).await,
                shard_id: ctx.shard_id.0 as u64,
                total_shards: total_shard_count(&ctx).await,
            };
            let data = executor::execute_code(ast, context, &state).await;
            send_event_response(ctx, &channel_id, &data).await;
        }
    }
}

pub struct Bot {
    pub commands: CommandMap,
    pub guild_id: Option<u64>,
    pub db: Db,
    pub bot_id: String,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);

        // Load presence from zbr.json
        let config_str = std::fs::read_to_string("zbr.json").unwrap_or_default();
        let config = serde_json::from_str::<crate::types::Config>(&config_str).unwrap_or_else(|_| {
            crate::types::Config {
                status: None,
                activity: None,
                logging: true,
            }
        });

        let status = match config.status.as_deref() {
            Some("dnd") => OnlineStatus::DoNotDisturb,
            Some("idle") => OnlineStatus::Idle,
            Some("invisible") => OnlineStatus::Invisible,
            _ => OnlineStatus::Online,
        };

        let activity = config.activity.as_ref().map(|a| {
            match a.activity_type.as_str() {
                "listening" => ActivityData::listening(a.name.clone()),
                "watching" => ActivityData::watching(a.name.clone()),
                "competing" => ActivityData::competing(a.name.clone()),
                _ => ActivityData::playing(a.name.clone()),
            }
        });

        // Small delay to ensure the session is ready for presence updates
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        ctx.set_presence(activity, status);

        let commands = self.commands.read().await;

        for cmd in commands.values() {
            match cmd.command_type {
                CommandType::Slash => {
                    let mut builder = CreateCommand::new(
                        cmd.trigger.trim_start_matches('/')
                    ).description(&cmd.description);

                    for opt in &cmd.options {
                        let option = CreateCommandOption::new(
                            opt.option_type.to_serenity_type(),
                            &opt.name,
                            &opt.description
                        ).required(opt.required);
                        builder = builder.add_option(option);
                    }

                    match cmd.scope {
                        CommandScope::Guild => {
                            if let Some(guild_id) = self.guild_id {
                                let gid = GuildId::new(guild_id);
                                if let Err(e) = gid.create_command(&ctx.http, builder).await {
                                    eprintln!(
                                        "Failed to register guild slash command {}: {}",
                                        cmd.name,
                                        e
                                    );
                                } else if config.logging {
                                    println!("Registered guild slash command: /{}", cmd.name);
                                }
                            } else {
                                eprintln!("Guild scope set but no GUILD_ID in .env");
                            }
                        }
                        CommandScope::Global => {
                            if
                                let Err(e) =
                                    serenity::model::application::Command::create_global_command(
                                        &ctx.http,
                                        builder
                                    ).await
                            {
                                eprintln!(
                                    "Failed to register global slash command {}: {}",
                                    cmd.name,
                                    e
                                );
                            } else if config.logging {
                                println!("Registered global slash command: /{}", cmd.name);
                            }
                        }
                        CommandScope::Both => {
                            if let Some(guild_id) = self.guild_id {
                                let gid = GuildId::new(guild_id);
                                if
                                    let Err(e) = gid.create_command(
                                        &ctx.http,
                                        builder.clone()
                                    ).await
                                {
                                    eprintln!(
                                        "Failed to register guild slash command {}: {}",
                                        cmd.name,
                                        e
                                    );
                                }
                            }
                            if
                                let Err(e) =
                                    serenity::model::application::Command::create_global_command(
                                        &ctx.http,
                                        builder
                                    ).await
                            {
                                eprintln!(
                                    "Failed to register global slash command {}: {}",
                                    cmd.name,
                                    e
                                );
                            } else if config.logging {
                                println!(
                                    "Registered both guild and global slash command: /{}",
                                    cmd.name
                                );
                            }
                        }
                    }
                }
                CommandType::Prefix => {}
                CommandType::Interaction => {}
                CommandType::Event => {}
                CommandType::SubSlash => {}
            }
        }

        // Register parent commands for subcommands
        let mut parent_commands: std::collections::HashMap<
            String,
            Vec<&Command>
        > = std::collections::HashMap::new();
        for cmd in commands.values() {
            if let CommandType::SubSlash = cmd.command_type {
                if let Some(space_pos) = cmd.trigger.find(' ') {
                    let parent = cmd.trigger[1..space_pos].to_string();
                    parent_commands.entry(parent).or_insert(Vec::new()).push(cmd);
                }
            }
        }

        for (parent_name, subs) in parent_commands {
            let mut builder = CreateCommand::new(&parent_name).description("Subcommands");

            // If there's a slash command with the same name, use its details
            if
                let Some(slash_cmd) = commands
                    .values()
                    .find(|c| {
                        matches!(c.command_type, CommandType::Slash) &&
                            c.trigger == format!("/{}", parent_name)
                    })
            {
                builder = builder.description(&slash_cmd.description);
                for opt in &slash_cmd.options {
                    builder = builder.add_option(
                        CreateCommandOption::new(
                            opt.option_type.to_serenity_type(),
                            &opt.name,
                            &opt.description
                        ).required(opt.required)
                    );
                }
            }

            for sub in subs {
                let parts: Vec<&str> = sub.trigger.split(' ').collect();
                if parts.len() >= 2 {
                    let sub_name = parts[1];
                    let mut sub_option = CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        sub_name,
                        &sub.description
                    );
                    for opt in &sub.options {
                        sub_option = sub_option.add_sub_option(
                            CreateCommandOption::new(
                                opt.option_type.to_serenity_type(),
                                &opt.name,
                                &opt.description
                            ).required(opt.required)
                        );
                    }
                    builder = builder.add_option(sub_option);
                }
            }

            // Register the command (guild only for now)
            if let Some(guild_id) = self.guild_id {
                let gid = GuildId::new(guild_id);
                if let Err(e) = gid.create_command(&ctx.http, builder).await {
                    eprintln!("Failed to register subcommand parent /{}: {}", parent_name, e);
                } else {
                    println!("Registered subcommand parent: /{}", parent_name);
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content.trim().to_string();

        // Auto-track message for spam detection
        if let Some(guild_id) = msg.guild_id {
            let has_link = content.contains("http://") || content.contains("https://");
            crate::db::log_spam_event(
                &self.db,
                &self.bot_id,
                &guild_id.to_string(),
                &msg.author.id.to_string(),
                &msg.channel_id.to_string(),
                has_link
            ).await;
        }

        let on_message_context = RunContext {
            author_id: msg.author.id.to_string(),
            username: msg.author.name.clone(),
            channel_id: msg.channel_id.to_string(),
            guild_id: msg.guild_id.map(|g| g.to_string()).unwrap_or_default(),
            message: content.clone(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onMessage".to_string(),
            trigger_message_id: Some(msg.id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onMessage", on_message_context).await;

        let mut trigger_cache = HashMap::new();
        let commands = self.commands.read().await;

        for (trigger, cmd) in commands.iter() {
            if let CommandType::Prefix = cmd.command_type {
                let resolved_trigger = if trigger.contains('Z') && trigger.contains('{') {
                    resolve_trigger_vars(
                        trigger,
                        &self.db,
                        &self.bot_id,
                        &msg,
                        &mut trigger_cache
                    ).await
                } else {
                    trigger.clone()
                };

                if
                    content.starts_with(&resolved_trigger) &&
                    ({
                        let rest = &content[resolved_trigger.len()..];
                        rest.is_empty() || rest.starts_with(char::is_whitespace)
                    })
                {
                    let context = RunContext {
                        author_id: msg.author.id.to_string(),
                        username: msg.author.name.clone(),
                        channel_id: msg.channel_id.to_string(),
                        guild_id: msg.guild_id.map(|g| g.to_string()).unwrap_or_default(),
                        message: content.clone(),
                        options: HashMap::new(),
                        options_list: Vec::new(),
                        trigger: Some(resolved_trigger),
                        command_name: cmd.name.clone(),
                        trigger_message_id: Some(msg.id.to_string()),
                        custom_id: None,
                        modal_values: HashMap::new(),
                        selected_values: vec![],
                    };

                    let ast = cmd.ast.clone();
                    drop(commands);

                    let state = ExecState {
                        db: self.db.clone(),
                        bot_id: self.bot_id.clone(),
                        http: Some(ctx.http.clone()),
                        cache: ctx.cache.clone(),
                        shard_latency: get_latency(&ctx).await,
                        shard_id: ctx.shard_id.0 as u64,
                        total_shards: total_shard_count(&ctx).await,
                    };

                    let data = executor::execute_code(ast, context, &state).await;

                    send_response(&ctx, &msg, &data).await;
                    return;
                }
            }
        }
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent
    ) {
        let message_content = if let Some(m) = &new {
            m.content.clone()
        } else if let Some(m) = &old_if_available {
            m.content.clone()
        } else {
            event.content.clone().unwrap_or_default()
        };

        let guild_id = new
            .as_ref()
            .and_then(|m| m.guild_id)
            .or_else(|| old_if_available.as_ref().and_then(|m| m.guild_id))
            .map(|g| g.to_string())
            .unwrap_or_default();

        let channel_id = new
            .as_ref()
            .map(|m| m.channel_id.to_string())
            .or_else(|| old_if_available.as_ref().map(|m| m.channel_id.to_string()))
            .unwrap_or_default();

        let trigger_message_id = new
            .as_ref()
            .map(|m| m.id.to_string())
            .or_else(|| old_if_available.as_ref().map(|m| m.id.to_string()));

        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id,
            guild_id,
            message: message_content,
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onMessageEdit".to_string(),
            trigger_message_id,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onMessageEdit", context).await;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: serenity::model::id::ChannelId,
        deleted_message_id: serenity::model::id::MessageId,
        guild_id: Option<GuildId>
    ) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: channel_id.to_string(),
            guild_id: guild_id.map(|g| g.to_string()).unwrap_or_default(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onMessageDelete".to_string(),
            trigger_message_id: Some(deleted_message_id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onMessageDelete", context).await;
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let context = RunContext {
            author_id: add_reaction.user_id.map(|u| u.to_string()).unwrap_or_default(),
            username: String::new(),
            channel_id: add_reaction.channel_id.to_string(),
            guild_id: add_reaction.guild_id.map(|g| g.to_string()).unwrap_or_default(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onReactionAdd".to_string(),
            trigger_message_id: Some(add_reaction.message_id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onReactionAdd", context).await;
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        let context = RunContext {
            author_id: removed_reaction.user_id.map(|u| u.to_string()).unwrap_or_default(),
            username: String::new(),
            channel_id: removed_reaction.channel_id.to_string(),
            guild_id: removed_reaction.guild_id.map(|g| g.to_string()).unwrap_or_default(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onReactionRemove".to_string(),
            trigger_message_id: Some(removed_reaction.message_id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onReactionRemove", context).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        // Auto-track join for raid detection
        crate::db::log_raid_event(
            &self.db,
            &self.bot_id,
            &new_member.guild_id.to_string(),
            &new_member.user.id.to_string()
        ).await;

        let context = RunContext {
            author_id: new_member.user.id.to_string(),
            username: new_member.user.name.clone(),
            channel_id: String::new(),
            guild_id: new_member.guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onMemberJoin".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onMemberJoin", context).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>
    ) {
        let context = RunContext {
            author_id: user.id.to_string(),
            username: user.name.clone(),
            channel_id: String::new(),
            guild_id: guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onMemberLeave".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onMemberLeave", context).await;
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        _old_if_available: Option<Member>,
        new: Option<Member>,
        _event: GuildMemberUpdateEvent
    ) {
        let (author_id, username, guild_id) = if let Some(member) = new {
            (member.user.id.to_string(), member.user.name.clone(), member.guild_id.to_string())
        } else {
            (String::new(), String::new(), String::new())
        };

        let context = RunContext {
            author_id,
            username,
            channel_id: String::new(),
            guild_id,
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onMemberUpdate".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onMemberUpdate", context).await;
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        let context = RunContext {
            author_id: banned_user.id.to_string(),
            username: banned_user.name.clone(),
            channel_id: String::new(),
            guild_id: guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onBanAdd".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onBanAdd", context).await;
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        let context = RunContext {
            author_id: unbanned_user.id.to_string(),
            username: unbanned_user.name.clone(),
            channel_id: String::new(),
            guild_id: guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onBanRemove".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onBanRemove", context).await;
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: String::new(),
            guild_id: new.guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onRoleCreate".to_string(),
            trigger_message_id: Some(new.id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onRoleCreate", context).await;
    }

    async fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        removed_role_id: serenity::model::id::RoleId,
        _removed_role_data_if_available: Option<Role>
    ) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: String::new(),
            guild_id: guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onRoleDelete".to_string(),
            trigger_message_id: Some(removed_role_id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onRoleDelete", context).await;
    }

    async fn guild_role_update(
        &self,
        ctx: Context,
        _old_data_if_available: Option<Role>,
        new: Role
    ) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: String::new(),
            guild_id: new.guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onRoleUpdate".to_string(),
            trigger_message_id: Some(new.id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onRoleUpdate", context).await;
    }

    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: channel.id.to_string(),
            guild_id: channel.guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onChannelCreate".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onChannelCreate", context).await;
    }

    async fn channel_delete(
        &self,
        ctx: Context,
        channel: GuildChannel,
        _messages: Option<Vec<Message>>
    ) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: channel.id.to_string(),
            guild_id: channel.guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onChannelDelete".to_string(),
            trigger_message_id: Some(channel.id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onChannelDelete", context).await;
    }

    async fn channel_update(&self, ctx: Context, _old: Option<GuildChannel>, new: GuildChannel) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: new.id.to_string(),
            guild_id: new.guild_id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onChannelUpdate".to_string(),
            trigger_message_id: Some(new.id.to_string()),
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onChannelUpdate", context).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        if is_new.unwrap_or(false) {
            let context = RunContext {
                author_id: String::new(),
                username: String::new(),
                channel_id: String::new(),
                guild_id: guild.id.to_string(),
                message: String::new(),
                options: HashMap::new(),
                options_list: Vec::new(),
                trigger: None,
                command_name: "onBotJoin".to_string(),
                trigger_message_id: None,
                custom_id: None,
                modal_values: HashMap::new(),
                selected_values: vec![],
            };

            self.run_event_command(&ctx, "onBotJoin", context).await;
        }
    }

    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, _full: Option<Guild>) {
        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: String::new(),
            guild_id: incomplete.id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onBotLeave".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onBotLeave", context).await;
    }

    async fn guild_update(
        &self,
        ctx: Context,
        _old_data_if_available: Option<Guild>,
        new_data: PartialGuild
    ) {
        if let Some(old_guild) = &_old_data_if_available {
            if let Some(new_boost_count) = new_data.premium_subscription_count {
                let old_boost_count = old_guild.premium_subscription_count.unwrap_or(0);
                let guild_id = new_data.id.to_string();

                if new_boost_count > old_boost_count {
                    let boost_context = RunContext {
                        author_id: String::new(),
                        username: String::new(),
                        channel_id: String::new(),
                        guild_id: guild_id.clone(),
                        message: String::new(),
                        options: HashMap::new(),
                        options_list: Vec::new(),
                        trigger: None,
                        command_name: "onBoostAdd".to_string(),
                        trigger_message_id: None,
                        custom_id: None,
                        modal_values: HashMap::new(),
                        selected_values: vec![],
                    };

                    self.run_event_command(&ctx, "onBoostAdd", boost_context).await;
                } else if new_boost_count < old_boost_count {
                    let unboost_context = RunContext {
                        author_id: String::new(),
                        username: String::new(),
                        channel_id: String::new(),
                        guild_id: guild_id.clone(),
                        message: String::new(),
                        options: HashMap::new(),
                        options_list: Vec::new(),
                        trigger: None,
                        command_name: "onBoostRemove".to_string(),
                        trigger_message_id: None,
                        custom_id: None,
                        modal_values: HashMap::new(),
                        selected_values: vec![],
                    };

                    self.run_event_command(&ctx, "onBoostRemove", unboost_context).await;
                }
            }
        }

        let context = RunContext {
            author_id: String::new(),
            username: String::new(),
            channel_id: String::new(),
            guild_id: new_data.id.to_string(),
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onGuildUpdate".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onGuildUpdate", context).await;
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, new: VoiceState) {
        let channel_id = new.channel_id.map(|c| c.to_string()).unwrap_or_default();
        let guild_id = new.guild_id.map(|g| g.to_string()).unwrap_or_default();

        let context = RunContext {
            author_id: new.user_id.to_string(),
            username: String::new(),
            channel_id,
            guild_id,
            message: String::new(),
            options: HashMap::new(),
            options_list: Vec::new(),
            trigger: None,
            command_name: "onVoiceStateUpdate".to_string(),
            trigger_message_id: None,
            custom_id: None,
            modal_values: HashMap::new(),
            selected_values: vec![],
        };

        self.run_event_command(&ctx, "onVoiceStateUpdate", context).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let (trigger, options_map, options_list) = build_command_trigger_and_options(
                    &command.data.name,
                    &command.data.options
                );
                let commands = self.commands.read().await;

                if let Some(cmd) = commands.get(&trigger) {
                    let context = RunContext {
                        author_id: command.user.id.to_string(),
                        username: command.user.name.clone(),
                        channel_id: command.channel_id.to_string(),
                        guild_id: command.guild_id.map(|g| g.to_string()).unwrap_or_default(),
                        message: String::new(),
                        options: options_map,
                        options_list,
                        trigger: None,
                        command_name: cmd.name.clone(),
                        trigger_message_id: None,
                        custom_id: None,
                        modal_values: HashMap::new(),
                        selected_values: vec![],
                    };

                    let ast = cmd.ast.clone();
                    drop(commands);

                    let state = ExecState {
                        db: self.db.clone(),
                        bot_id: self.bot_id.clone(),
                        http: Some(ctx.http.clone()),
                        cache: ctx.cache.clone(),
                        shard_latency: get_latency(&ctx).await,
                        shard_id: ctx.shard_id.0 as u64,
                        total_shards: total_shard_count(&ctx).await,
                    };

                    let data = executor::execute_code(ast, context, &state).await;

                    let built_embeds = build_embeds(&data.embeds);
                    let text_content = resolve_text_content(&data);

                    if built_embeds.is_empty() && text_content.is_none() {
                        return;
                    }

                    let mut msg = CreateInteractionResponseMessage::new();
                    if let Some(text) = text_content {
                        msg = msg.content(text);
                    }
                    for e in built_embeds {
                        msg = msg.add_embed(e);
                    }

                    // Apply allowed mentions if set by ZallowUserMentions / ZallowRoleMentions
                    if data.allowed_user_mentions.is_some() || data.allowed_role_mentions.is_some() {
                        let mut allowed = CreateAllowedMentions::new();
                        if let Some(user_ids) = &data.allowed_user_mentions {
                            let ids: Vec<u64> = user_ids
                                .iter()
                                .filter_map(|id| id.parse().ok())
                                .collect();
                            allowed = allowed.users(ids);
                        }
                        if let Some(role_ids) = &data.allowed_role_mentions {
                            let ids: Vec<u64> = role_ids
                                .iter()
                                .filter_map(|id| id.parse().ok())
                                .collect();
                            allowed = allowed.roles(ids);
                        }
                        msg = msg.allowed_mentions(allowed);
                    }

                    if data.ephemeral {
                        msg = msg.ephemeral(true);
                    }

                    // Attach components
                    for row in build_components(&data.components) {
                        msg = msg.components(vec![row]);
                    }

                    let response = CreateInteractionResponse::Message(msg);
                    if let Err(e) = command.create_response(&ctx.http, response).await {
                        eprintln!("Failed to respond to slash command: {}", e);
                    }

                    // Apply pending reactions to the bot's interaction response
                    if !data.pending_reactions.is_empty() {
                        if let Ok(sent_msg) = command.get_response(&ctx.http).await {
                            for emoji_str in &data.pending_reactions {
                                let reaction = parse_reaction_type(emoji_str);
                                if let Err(e) = sent_msg.react(&ctx.http, reaction).await {
                                    // Error ignored intentionally
                                }
                            }
                        }
                    }
                }
            }

            Interaction::Component(component) => {
                let custom_id = component.data.custom_id.clone();
                let commands = self.commands.read().await;

                // Look up specific handler first, then catch-all
                let specific_key = format!("onInteraction{{{}}}", custom_id);
                let catchall_key = "onInteraction".to_string();

                let cmd = commands
                    .get(&specific_key)
                    .or_else(|| commands.get(&catchall_key))
                    .filter(|c| matches!(c.command_type, crate::types::CommandType::Interaction));

                if let Some(cmd) = cmd {
                    // Collect selected values for select menus
                    let selected_values: Vec<String> = {
                        use serenity::model::application::ComponentInteractionDataKind;
                        match &component.data.kind {
                            ComponentInteractionDataKind::StringSelect { values } => values.clone(),
                            ComponentInteractionDataKind::UserSelect { values } => {
                                values
                                    .iter()
                                    .map(|id| id.to_string())
                                    .collect()
                            }
                            ComponentInteractionDataKind::RoleSelect { values } => {
                                values
                                    .iter()
                                    .map(|id| id.to_string())
                                    .collect()
                            }
                            ComponentInteractionDataKind::MentionableSelect { values } => {
                                values
                                    .iter()
                                    .map(|id| id.to_string())
                                    .collect()
                            }
                            ComponentInteractionDataKind::ChannelSelect { values } => {
                                values
                                    .iter()
                                    .map(|id| id.to_string())
                                    .collect()
                            }
                            _ => vec![],
                        }
                    };

                    let context = RunContext {
                        author_id: component.user.id.to_string(),
                        username: component.user.name.clone(),
                        channel_id: component.channel_id.to_string(),
                        guild_id: component.guild_id.map(|g| g.to_string()).unwrap_or_default(),
                        message: String::new(),
                        options: HashMap::new(),
                        options_list: selected_values.clone(),
                        trigger: None,
                        command_name: cmd.name.clone(),
                        trigger_message_id: Some(component.message.id.to_string()),
                        custom_id: Some(custom_id.clone()),
                        modal_values: HashMap::new(),
                        selected_values: selected_values.clone(),
                    };

                    let ast = cmd.ast.clone();
                    drop(commands);

                    let state = ExecState {
                        db: self.db.clone(),
                        bot_id: self.bot_id.clone(),
                        http: Some(ctx.http.clone()),
                        cache: ctx.cache.clone(),
                        shard_latency: get_latency(&ctx).await,
                        shard_id: ctx.shard_id.0 as u64,
                        total_shards: total_shard_count(&ctx).await,
                    };

                    let data = executor::execute_code(ast, context, &state).await;

                    // If Zdefer was called, send a deferred acknowledgment
                    if data.components.deferred {
                        let _ = component.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Acknowledge
                        ).await;
                        return;
                    }

                    // If a modal was built, respond with the modal instead of a message
                    if let Some(modal) = &data.components.modal {
                        let mut rows: Vec<CreateActionRow> = Vec::new();
                        for field in &modal.fields {
                            use serenity::model::application::InputTextStyle;
                            let style = if field.style == "paragraph" {
                                InputTextStyle::Paragraph
                            } else {
                                InputTextStyle::Short
                            };
                            let mut input = CreateInputText::new(
                                style,
                                &field.label,
                                &field.field_id
                            ).required(field.required);
                            if let Some(min) = field.min_length {
                                input = input.min_length(min as u16);
                            }
                            if let Some(max) = field.max_length {
                                input = input.max_length(max as u16);
                            }
                            if let Some(ph) = &field.placeholder {
                                input = input.placeholder(ph);
                            }
                            if let Some(val) = &field.value {
                                input = input.value(val);
                            }
                            rows.push(CreateActionRow::InputText(input));
                        }
                        let modal_builder = CreateModal::new(
                            &modal.modal_id,
                            &modal.title
                        ).components(rows);
                        let response = CreateInteractionResponse::Modal(modal_builder);
                        if let Err(e) = component.create_response(&ctx.http, response).await {
                            eprintln!("Failed to show modal: {}", e);
                        }
                        return;
                    }

                    let built_embeds = build_embeds(&data.embeds);
                    let text_content = resolve_text_content(&data);

                    // If nothing to send, just acknowledge
                    if built_embeds.is_empty() && text_content.is_none() {
                        let _ = component.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Acknowledge
                        ).await;
                        return;
                    }

                    let response = if data.components.update_message {
                        let mut msg = CreateInteractionResponseMessage::new();
                        if let Some(text) = text_content {
                            msg = msg.content(text);
                        }
                        for e in built_embeds {
                            msg = msg.add_embed(e);
                        }
                        for row in build_components(&data.components) {
                            msg = msg.components(vec![row]);
                        }
                        CreateInteractionResponse::UpdateMessage(msg)
                    } else {
                        let mut msg = CreateInteractionResponseMessage::new();
                        if let Some(text) = text_content {
                            msg = msg.content(text);
                        }
                        for e in built_embeds {
                            msg = msg.add_embed(e);
                        }
                        if data.ephemeral {
                            msg = msg.ephemeral(true);
                        }
                        for row in build_components(&data.components) {
                            msg = msg.components(vec![row]);
                        }
                        CreateInteractionResponse::Message(msg)
                    };

                    if let Err(e) = component.create_response(&ctx.http, response).await {
                        eprintln!("Failed to respond to component interaction: {}", e);
                    }
                } else {
                    drop(commands);
                    // No handler — acknowledge silently so Discord doesn't show "interaction failed"
                    let _ = component.create_response(
                        &ctx.http,
                        CreateInteractionResponse::Acknowledge
                    ).await;
                }
            }

            Interaction::Modal(modal) => {
                let custom_id = modal.data.custom_id.clone();
                let commands = self.commands.read().await;

                let specific_key = format!("onInteraction{{{}}}", custom_id);
                let catchall_key = "onInteraction".to_string();

                let cmd = commands
                    .get(&specific_key)
                    .or_else(|| commands.get(&catchall_key))
                    .filter(|c| matches!(c.command_type, crate::types::CommandType::Interaction));

                if let Some(cmd) = cmd {
                    // Collect modal field values
                    let mut modal_values = HashMap::new();
                    for row in &modal.data.components {
                        for comp in &row.components {
                            use serenity::model::application::ActionRowComponent;
                            if let ActionRowComponent::InputText(input) = comp {
                                modal_values.insert(
                                    input.custom_id.clone(),
                                    input.value.clone().unwrap_or_default()
                                );
                            }
                        }
                    }

                    let context = RunContext {
                        author_id: modal.user.id.to_string(),
                        username: modal.user.name.clone(),
                        channel_id: modal.channel_id.to_string(),
                        guild_id: modal.guild_id.map(|g| g.to_string()).unwrap_or_default(),
                        message: String::new(),
                        options: HashMap::new(),
                        options_list: Vec::new(),
                        trigger: None,
                        command_name: cmd.name.clone(),
                        trigger_message_id: None,
                        custom_id: Some(custom_id.clone()),
                        modal_values,
                        selected_values: vec![],
                    };

                    let ast = cmd.ast.clone();
                    drop(commands);

                    let state = ExecState {
                        db: self.db.clone(),
                        bot_id: self.bot_id.clone(),
                        http: Some(ctx.http.clone()),
                        cache: ctx.cache.clone(),
                        shard_latency: get_latency(&ctx).await,
                        shard_id: ctx.shard_id.0 as u64,
                        total_shards: total_shard_count(&ctx).await,
                    };

                    let data = executor::execute_code(ast, context, &state).await;

                    let built_embeds = build_embeds(&data.embeds);
                    let text_content = resolve_text_content(&data);

                    if built_embeds.is_empty() && text_content.is_none() {
                        let _ = modal.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Acknowledge
                        ).await;
                        return;
                    }

                    let mut msg = CreateInteractionResponseMessage::new();
                    if let Some(text) = text_content {
                        msg = msg.content(text);
                    }
                    for e in built_embeds {
                        msg = msg.add_embed(e);
                    }
                    if data.ephemeral {
                        msg = msg.ephemeral(true);
                    }

                    let response = CreateInteractionResponse::Message(msg);
                    if let Err(e) = modal.create_response(&ctx.http, response).await {
                        eprintln!("Failed to respond to modal: {}", e);
                    }
                } else {
                    drop(commands);
                    let _ = modal.create_response(
                        &ctx.http,
                        CreateInteractionResponse::Acknowledge
                    ).await;
                }
            }

            _ => {}
        }
    }
}

fn resolve_text_content(data: &RunResponse) -> Option<String> {
    if !data.errors.is_empty() {
        return Some(data.errors.join("
"));
    }
    if !data.output.is_empty() {
        return Some(data.output.join("
"));
    }
    None
}

fn build_command_trigger_and_options(
    command_name: &str,
    options: &[serenity::model::application::CommandDataOption]
) -> (String, HashMap<String, String>, Vec<String>) {
    use serenity::model::application::CommandDataOptionValue;

    let mut options_map = HashMap::new();
    let mut options_list = Vec::new();
    let mut trigger = format!("/{}", command_name);

    if let Some(first_option) = options.first() {
        match &first_option.value {
            CommandDataOptionValue::SubCommand(nested) => {
                trigger = format!("{} {}", trigger, first_option.name);
                for opt in nested {
                    flatten_option_values(opt, &mut options_map, &mut options_list);
                }
            }
            CommandDataOptionValue::SubCommandGroup(nested) => {
                if let Some(sub) = nested.first() {
                    if let CommandDataOptionValue::SubCommand(sub_nested) = &sub.value {
                        trigger = format!("{} {} {}", trigger, first_option.name, sub.name);
                        for opt in sub_nested {
                            flatten_option_values(opt, &mut options_map, &mut options_list);
                        }
                    }
                }
            }
            _ => {
                for opt in options {
                    flatten_option_values(opt, &mut options_map, &mut options_list);
                }
            }
        }
    }

    (trigger, options_map, options_list)
}

fn flatten_option_values(
    option: &serenity::model::application::CommandDataOption,
    values: &mut HashMap<String, String>,
    list: &mut Vec<String>
) {
    use serenity::model::application::CommandDataOptionValue;

    let val = match &option.value {
        CommandDataOptionValue::String(s) => s.clone(),
        CommandDataOptionValue::Integer(i) => i.to_string(),
        CommandDataOptionValue::Number(n) => n.to_string(),
        CommandDataOptionValue::Boolean(b) => b.to_string(),
        CommandDataOptionValue::User(u) => u.to_string(),
        CommandDataOptionValue::Channel(c) => c.to_string(),
        CommandDataOptionValue::Role(r) => r.to_string(),
        CommandDataOptionValue::Attachment(a) => a.to_string(),
        // SubCommand / SubCommandGroup / Autocomplete / Unknown — not a scalar value
        _ => {
            return;
        }
    };
    values.insert(option.name.clone(), val.clone());
    list.push(val);
}

async fn send_response(ctx: &Context, msg: &Message, data: &RunResponse) {
    let built_embeds = build_embeds(&data.embeds);
    let text_content = resolve_text_content(data);

    if built_embeds.is_empty() && text_content.is_none() {
        return;
    }

    // ZuseChannel overrides the destination channel
    let target_channel = if let Some(cid_str) = &data.use_channel {
        if let Ok(cid) = cid_str.parse::<u64>() {
            serenity::model::id::ChannelId::new(cid)
        } else {
            msg.channel_id
        }
    } else {
        msg.channel_id
    };

    let mut message = CreateMessage::new();
    if let Some(text) = text_content {
        message = message.content(text);
    }
    for e in built_embeds {
        message = message.add_embed(e);
    }
    // Only add reply reference when sending to the original channel
    if data.should_reply && data.use_channel.is_none() {
        message = message.reference_message(msg);
    }

    // Apply allowed mentions if set by ZallowUserMentions / ZallowRoleMentions
    if data.allowed_user_mentions.is_some() || data.allowed_role_mentions.is_some() {
        let mut allowed = CreateAllowedMentions::new();
        if let Some(user_ids) = &data.allowed_user_mentions {
            let ids: Vec<u64> = user_ids
                .iter()
                .filter_map(|id| id.parse().ok())
                .collect();
            allowed = allowed.users(ids);
        }
        if let Some(role_ids) = &data.allowed_role_mentions {
            let ids: Vec<u64> = role_ids
                .iter()
                .filter_map(|id| id.parse().ok())
                .collect();
            allowed = allowed.roles(ids);
        }
        message = message.allowed_mentions(allowed);
    }

    // Attach components
    for row in build_components(&data.components) {
        message = message.components(vec![row]);
    }

    let sent_result = target_channel.send_message(&ctx.http, message).await;
    let sent = sent_result.ok();

    // Apply pending reactions to the bot's response
    if let Some(sent_msg) = sent {
        for emoji_str in &data.pending_reactions {
            let reaction = parse_reaction_type(emoji_str);
            sent_msg.react(&ctx.http, reaction).await.ok();
        }
    }
}

async fn send_event_response(ctx: &Context, default_channel_id: &str, data: &RunResponse) {
    let built_embeds = build_embeds(&data.embeds);
    let text_content = resolve_text_content(data);

    if built_embeds.is_empty() && text_content.is_none() {
        return;
    }

    let target_channel = if let Some(cid_str) = &data.use_channel {
        if let Ok(cid) = cid_str.parse::<u64>() {
            serenity::model::id::ChannelId::new(cid)
        } else if let Ok(cid) = default_channel_id.parse::<u64>() {
            serenity::model::id::ChannelId::new(cid)
        } else {
            return;
        }
    } else if let Ok(cid) = default_channel_id.parse::<u64>() {
        serenity::model::id::ChannelId::new(cid)
    } else {
        return;
    };

    let mut message = CreateMessage::new();
    if let Some(text) = text_content {
        message = message.content(text);
    }
    for e in built_embeds {
        message = message.add_embed(e);
    }

    if data.allowed_user_mentions.is_some() || data.allowed_role_mentions.is_some() {
        let mut allowed = CreateAllowedMentions::new();
        if let Some(user_ids) = &data.allowed_user_mentions {
            let ids: Vec<u64> = user_ids
                .iter()
                .filter_map(|id| id.parse().ok())
                .collect();
            allowed = allowed.users(ids);
        }
        if let Some(role_ids) = &data.allowed_role_mentions {
            let ids: Vec<u64> = role_ids
                .iter()
                .filter_map(|id| id.parse().ok())
                .collect();
            allowed = allowed.roles(ids);
        }
        message = message.allowed_mentions(allowed);
    }

    for row in build_components(&data.components) {
        message = message.components(vec![row]);
    }

    let sent_result = target_channel.send_message(&ctx.http, message).await;
    if let Ok(sent_msg) = sent_result {
        for emoji_str in &data.pending_reactions {
            let reaction = parse_reaction_type(emoji_str);
            sent_msg.react(&ctx.http, reaction).await.ok();
        }
    }
}

fn build_components(data: &ComponentData) -> Vec<CreateActionRow> {
    use serenity::model::application::ButtonStyle;

    let mut rows: Vec<CreateActionRow> = Vec::new();

    // ── Buttons ───────────────────────────────────────────────────────────────
    if !data.buttons.is_empty() {
        let mut current_row: Vec<CreateButton> = Vec::new();

        for btn in &data.buttons {
            if btn.new_row && !current_row.is_empty() {
                rows.push(CreateActionRow::Buttons(current_row));
                current_row = Vec::new();
            }
            // Max 5 buttons per row
            if current_row.len() == 5 {
                rows.push(CreateActionRow::Buttons(current_row));
                current_row = Vec::new();
            }

            let style = match btn.style.as_str() {
                "primary" => ButtonStyle::Primary,
                "success" => ButtonStyle::Success,
                "danger" => ButtonStyle::Danger,
                _ => ButtonStyle::Secondary,
            };

            let mut b = if btn.style == "link" {
                CreateButton::new_link(&btn.custom_id)
            } else {
                CreateButton::new(&btn.custom_id).style(style)
            };

            b = b.label(&btn.label).disabled(btn.disabled);

            if let Some(emoji_str) = &btn.emoji {
                if let Ok(id) = emoji_str.parse::<u64>() {
                    b = b.emoji(serenity::model::id::EmojiId::new(id));
                } else if let Some(c) = emoji_str.chars().next() {
                    // Unicode emoji — use the first char
                    b = b.emoji(c);
                }
            }

            current_row.push(b);
        }

        if !current_row.is_empty() {
            rows.push(CreateActionRow::Buttons(current_row));
        }
    }

    if let Some(sm) = &data.select_menu {
        let kind = match sm.kind.as_str() {
            "user" =>
                CreateSelectMenuKind::User {
                    default_users: None,
                },
            "role" =>
                CreateSelectMenuKind::Role {
                    default_roles: None,
                },
            "mentionable" =>
                CreateSelectMenuKind::Mentionable {
                    default_users: None,
                    default_roles: None,
                },
            _ => {
                let options: Vec<CreateSelectMenuOption> = sm.options
                    .iter()
                    .map(|o| {
                        let mut opt = CreateSelectMenuOption::new(&o.label, &o.value);
                        if let Some(desc) = &o.description {
                            opt = opt.description(desc);
                        }
                        if o.default {
                            opt = opt.default_selection(true);
                        }
                        opt
                    })
                    .collect();
                CreateSelectMenuKind::String { options }
            }
        };

        let mut menu = CreateSelectMenu::new(&sm.menu_id, kind)
            .min_values(sm.min_values)
            .max_values(sm.max_values);

        if let Some(ph) = &sm.placeholder {
            menu = menu.placeholder(ph);
        }

        rows.push(CreateActionRow::SelectMenu(menu));
    }

    rows
}

fn build_embeds(embeds: &[EmbedData]) -> Vec<CreateEmbed> {
    embeds
        .iter()
        .map(|data| {
            let mut embed = CreateEmbed::new();
            if let Some(t) = &data.title {
                embed = embed.title(t);
            }
            if let Some(u) = &data.title_url {
                embed = embed.url(u);
            }
            if let Some(d) = &data.description {
                embed = embed.description(d);
            }
            if let Some(c) = data.color {
                embed = embed.color(c);
            }
            if let Some(t) = &data.thumbnail {
                embed = embed.thumbnail(t);
            }
            if let Some(i) = &data.image {
                embed = embed.image(i);
            }
            if let Some(f) = &data.footer {
                let mut footer = CreateEmbedFooter::new(f);
                if let Some(fi) = &data.footer_icon {
                    footer = footer.icon_url(fi);
                }
                embed = embed.footer(footer);
            }
            if let Some(a) = &data.author {
                let mut author = CreateEmbedAuthor::new(a);
                if let Some(ai) = &data.author_icon {
                    author = author.icon_url(ai);
                }
                if let Some(au) = &data.author_url {
                    author = author.url(au);
                }
                embed = embed.author(author);
            }
            if data.timestamp {
                embed = embed.timestamp(serenity::model::Timestamp::now());
            }
            for field in &data.fields {
                embed = embed.field(&field.name, &field.value, field.inline);
            }
            embed
        })
        .collect()
}

pub fn parse_reaction_type(s: &str) -> ReactionType {
    let s = s.trim();
    // Custom emoji: <:name:id> or <a:name:id>
    if s.starts_with('<') && s.ends_with('>') {
        let inner = &s[1..s.len() - 1];
        let animated = inner.starts_with("a:");
        let inner = if animated {
            &inner[2..]
        } else if inner.starts_with(':') {
            &inner[1..]
        } else {
            inner
        };
        if let Some(colon) = inner.rfind(':') {
            let name = &inner[..colon];
            let id_str = &inner[colon + 1..];
            if let Ok(id) = id_str.parse::<u64>() {
                return ReactionType::Custom {
                    animated,
                    id: EmojiId::new(id),
                    name: Some(name.to_string().into()),
                };
            }
        }
    }
    // Raw numeric ID
    if let Ok(id) = s.parse::<u64>() {
        return ReactionType::Custom {
            animated: false,
            id: EmojiId::new(id),
            name: None,
        };
    }
    // Unicode emoji
    ReactionType::Unicode(s.to_string().into())
}
