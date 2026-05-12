use crate::types::{CommandMap, CommandScope, CommandType, Db};
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::builder::{
    CreateAllowedMentions, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, CreateActionRow, CreateModal, CreateInputText,
};
use serenity::model::application::Interaction;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::gateway::Ready;
use serenity::model::id::{EmojiId, GuildId};
use serenity::prelude::*;
use std::collections::HashMap;

/// Serializable payload sent to the /run HTTP endpoint.
/// This is NOT the same as DiscordContext — it's just the JSON body.
#[derive(Serialize)]
struct RunRequest {
    code: String,
    context: RunContext,
}

/// The JSON-serializable context fields sent over HTTP to the runtime server.
#[derive(Serialize)]
struct RunContext {
    author_id: String,
    username: String,
    channel_id: String,
    guild_id: String,
    message: String,
    options: HashMap<String, String>,
    options_list: Vec<String>,
    trigger: Option<String>,
    command_name: String,
    trigger_message_id: Option<String>,
    #[serde(default)]
    custom_id: Option<String>,
    #[serde(default)]
    modal_values: HashMap<String, String>,
    #[serde(default)]
    selected_values: Vec<String>,
}

#[derive(Deserialize)]
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
    #[serde(default)]
    components: ComponentData,
}

#[derive(Deserialize, Default)]
struct ComponentData {
    #[serde(default)]
    buttons: Vec<ButtonData>,
    select_menu: Option<SelectMenuData>,
    modal: Option<ModalData>,
    #[serde(default)]
    deferred: bool,
}

#[derive(Deserialize)]
struct ButtonData {
    custom_id: String,
    label: String,
    style: String,
    #[serde(default)]
    disabled: bool,
    emoji: Option<String>,
    #[serde(default)]
    new_row: bool,
}

#[derive(Deserialize)]
struct SelectMenuData {
    menu_id: String,
    #[serde(default = "default_string_kind")]
    kind: String,
    #[serde(default = "default_one")]
    min_values: u8,
    #[serde(default = "default_one")]
    max_values: u8,
    placeholder: Option<String>,
    #[serde(default)]
    options: Vec<SelectOptionData>,
}

fn default_string_kind() -> String { "string".to_string() }

fn default_one() -> u8 { 1 }

#[derive(Deserialize)]
struct SelectOptionData {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<String>,
    #[serde(default)]
    default: bool,
}

#[derive(Deserialize)]
struct ModalData {
    modal_id: String,
    title: String,
    #[serde(default)]
    fields: Vec<ModalFieldData>,
}

#[derive(Deserialize)]
struct ModalFieldData {
    field_id: String,
    label: String,
    #[serde(default = "default_short")]
    style: String,
    min_length: Option<u32>,
    max_length: Option<u32>,
    #[serde(default = "default_true")]
    required: bool,
    placeholder: Option<String>,
    value: Option<String>,
}

fn default_short() -> String { "short".to_string() }
fn default_true() -> bool { true }

#[derive(Deserialize)]
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

#[derive(Deserialize)]
struct EmbedFieldData {
    name: String,
    value: String,
    inline: bool,
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

        let commands = self.commands.read().await;

        for cmd in commands.values() {
            match cmd.command_type {
                CommandType::Slash => {
                    let mut builder = CreateCommand::new(cmd.trigger.trim_start_matches('/'))
                        .description(&cmd.description);

                    for opt in &cmd.options {
                        let option = CreateCommandOption::new(
                            opt.option_type.to_serenity_type(),
                            &opt.name,
                            &opt.description,
                        )
                        .required(opt.required);
                        builder = builder.add_option(option);
                    }

                    match cmd.scope {
                        CommandScope::Guild => {
                            if let Some(guild_id) = self.guild_id {
                                let gid = GuildId::new(guild_id);
                                if let Err(e) = gid.create_command(&ctx.http, builder).await {
                                    eprintln!(
                                        "Failed to register guild slash command {}: {}",
                                        cmd.name, e
                                    );
                                } else {
                                    println!("Registered guild slash command: /{}", cmd.name);
                                }
                            } else {
                                eprintln!("Guild scope set but no GUILD_ID in .env");
                            }
                        }
                        CommandScope::Global => {
                            if let Err(e) =
                                serenity::model::application::Command::create_global_command(
                                    &ctx.http, builder,
                                )
                                .await
                            {
                                eprintln!(
                                    "Failed to register global slash command {}: {}",
                                    cmd.name, e
                                );
                            } else {
                                println!("Registered global slash command: /{}", cmd.name);
                            }
                        }
                        CommandScope::Both => {
                            if let Some(guild_id) = self.guild_id {
                                let gid = GuildId::new(guild_id);
                                if let Err(e) = gid.create_command(&ctx.http, builder.clone()).await
                                {
                                    eprintln!(
                                        "Failed to register guild slash command {}: {}",
                                        cmd.name, e
                                    );
                                }
                            }
                            if let Err(e) =
                                serenity::model::application::Command::create_global_command(
                                    &ctx.http, builder,
                                )
                                .await
                            {
                                eprintln!(
                                    "Failed to register global slash command {}: {}",
                                    cmd.name, e
                                );
                            }
                            println!(
                                "Registered both guild and global slash command: /{}",
                                cmd.name
                            );
                        }
                    }
                }
                CommandType::Prefix => {}
                CommandType::Interaction => {}
                CommandType::Event => {}
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content.trim().to_string();
        let commands = self.commands.read().await;

        for (trigger, cmd) in commands.iter() {
            if let CommandType::Prefix = cmd.command_type {
                if content.starts_with(trigger.as_str()) && {
                    let rest = &content[trigger.len()..];
                    rest.is_empty() || rest.starts_with(char::is_whitespace)
                } {
                    let context = RunContext {
                        author_id: msg.author.id.to_string(),
                        username: msg.author.name.clone(),
                        channel_id: msg.channel_id.to_string(),
                        guild_id: msg.guild_id.map(|g| g.to_string()).unwrap_or_default(),
                        message: content.clone(),
                        options: HashMap::new(),
                        options_list: Vec::new(),
                        trigger: Some(trigger.clone()),
                        command_name: cmd.name.clone(),
                        trigger_message_id: Some(msg.id.to_string()),
                        custom_id: None,
                        modal_values: HashMap::new(),
                        selected_values: vec![],
                    };

                    let code = cmd.code.clone();
                    drop(commands);

                    let data = match call_runtime(code, context).await {
                        Some(d) => d,
                        None => return,
                    };

                    send_response(&ctx, &msg, &data).await;
                    return;
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let trigger = format!("/{}", command.data.name);
                let commands = self.commands.read().await;

                if let Some(cmd) = commands.get(&trigger) {
                    let mut options_map = HashMap::new();
                    let mut options_list = Vec::new();
                    for opt in &command.data.options {
                        use serenity::model::application::CommandDataOptionValue;
                        let val = match &opt.value {
                            CommandDataOptionValue::String(s) => s.clone(),
                            CommandDataOptionValue::Integer(i) => i.to_string(),
                            CommandDataOptionValue::Number(n) => n.to_string(),
                            CommandDataOptionValue::Boolean(b) => b.to_string(),
                            CommandDataOptionValue::User(u) => u.to_string(),
                            CommandDataOptionValue::Channel(c) => c.to_string(),
                            CommandDataOptionValue::Role(r) => r.to_string(),
                            CommandDataOptionValue::Attachment(a) => a.to_string(),
                            _ => String::new(),
                        };
                        options_map.insert(opt.name.clone(), val.clone());
                        options_list.push(val);
                    }

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

                    let code = cmd.code.clone();
                    drop(commands);

                    let data = match call_runtime(code, context).await {
                        Some(d) => d,
                        None => return,
                    };

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
                            let ids: Vec<u64> =
                                user_ids.iter().filter_map(|id| id.parse().ok()).collect();
                            allowed = allowed.users(ids);
                        }
                        if let Some(role_ids) = &data.allowed_role_mentions {
                            let ids: Vec<u64> =
                                role_ids.iter().filter_map(|id| id.parse().ok()).collect();
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
                                sent_msg.react(&ctx.http, reaction).await.ok();
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

                let cmd = commands.get(&specific_key)
                    .or_else(|| commands.get(&catchall_key))
                    .filter(|c| matches!(c.command_type, crate::types::CommandType::Interaction));

                if let Some(cmd) = cmd {
                    // Collect selected values for select menus
                    let selected_values: Vec<String> = {
                        use serenity::model::application::ComponentInteractionDataKind;
                        match &component.data.kind {
                            ComponentInteractionDataKind::StringSelect { values } => values.clone(),
                            ComponentInteractionDataKind::UserSelect { values } => {
                                values.iter().map(|id| id.to_string()).collect()
                            }
                            ComponentInteractionDataKind::RoleSelect { values } => {
                                values.iter().map(|id| id.to_string()).collect()
                            }
                            ComponentInteractionDataKind::MentionableSelect { values } => {
                                values.iter().map(|id| id.to_string()).collect()
                            }
                            ComponentInteractionDataKind::ChannelSelect { values } => {
                                values.iter().map(|id| id.to_string()).collect()
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

                    let code = cmd.code.clone();
                    drop(commands);

                    let data = match call_runtime(code, context).await {
                        Some(d) => d,
                        None => {
                            // Still must acknowledge the interaction
                            let _ = component.create_response(&ctx.http,
                                CreateInteractionResponse::Acknowledge).await;
                            return;
                        }
                    };

                    // If Zdefer was called, send a deferred acknowledgment
                    if data.components.deferred {
                        let _ = component.create_response(&ctx.http,
                            CreateInteractionResponse::Acknowledge).await;
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
                            let mut input = CreateInputText::new(style, &field.label, &field.field_id)
                                .required(field.required);
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
                        let modal_builder = CreateModal::new(&modal.modal_id, &modal.title)
                            .components(rows);
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
                        let _ = component.create_response(&ctx.http,
                            CreateInteractionResponse::Acknowledge).await;
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
                    for row in build_components(&data.components) {
                        msg = msg.components(vec![row]);
                    }

                    let response = CreateInteractionResponse::Message(msg);
                    if let Err(e) = component.create_response(&ctx.http, response).await {
                        eprintln!("Failed to respond to component interaction: {}", e);
                    }
                } else {
                    drop(commands);
                    // No handler — acknowledge silently so Discord doesn't show "interaction failed"
                    let _ = component.create_response(&ctx.http,
                        CreateInteractionResponse::Acknowledge).await;
                }
            }

            Interaction::Modal(modal) => {
                let custom_id = modal.data.custom_id.clone();
                let commands = self.commands.read().await;

                let specific_key = format!("onInteraction{{{}}}", custom_id);
                let catchall_key = "onInteraction".to_string();

                let cmd = commands.get(&specific_key)
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
                                    input.value.clone().unwrap_or_default(),
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

                    let code = cmd.code.clone();
                    drop(commands);

                    let data = match call_runtime(code, context).await {
                        Some(d) => d,
                        None => {
                            let _ = modal.create_response(&ctx.http,
                                CreateInteractionResponse::Acknowledge).await;
                            return;
                        }
                    };

                    let built_embeds = build_embeds(&data.embeds);
                    let text_content = resolve_text_content(&data);

                    if built_embeds.is_empty() && text_content.is_none() {
                        let _ = modal.create_response(&ctx.http,
                            CreateInteractionResponse::Acknowledge).await;
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
                    let _ = modal.create_response(&ctx.http,
                        CreateInteractionResponse::Acknowledge).await;
                }
            }

            _ => {}
        }
    }
}

fn resolve_text_content(data: &RunResponse) -> Option<String> {
    if !data.errors.is_empty() {
        return Some(data.errors.join("\n"));
    }
    if !data.output.is_empty() {
        return Some(data.output.join("\n"));
    }
    None
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
            let ids: Vec<u64> = user_ids.iter().filter_map(|id| id.parse().ok()).collect();
            allowed = allowed.users(ids);
        }
        if let Some(role_ids) = &data.allowed_role_mentions {
            let ids: Vec<u64> = role_ids.iter().filter_map(|id| id.parse().ok()).collect();
            allowed = allowed.roles(ids);
        }
        message = message.allowed_mentions(allowed);
    }

    // Attach components
    for row in build_components(&data.components) {
        message = message.components(vec![row]);
    }

    let sent = target_channel.send_message(&ctx.http, message).await.ok();

    // Apply pending reactions to the bot's response
    if let Some(sent_msg) = sent {
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
                "primary"   => ButtonStyle::Primary,
                "success"   => ButtonStyle::Success,
                "danger"    => ButtonStyle::Danger,
                _           => ButtonStyle::Secondary,
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

    // ── String/User/Role/Mentionable select menu ──────────────────────────────
    if let Some(sm) = &data.select_menu {
        let kind = match sm.kind.as_str() {
            "user"        => CreateSelectMenuKind::User { default_users: None },
            "role"        => CreateSelectMenuKind::Role { default_roles: None },
            "mentionable" => CreateSelectMenuKind::Mentionable { default_users: None, default_roles: None },
            _             => {
                let options: Vec<CreateSelectMenuOption> = sm.options.iter().map(|o| {
                    let mut opt = CreateSelectMenuOption::new(&o.label, &o.value);
                    if let Some(desc) = &o.description {
                        opt = opt.description(desc);
                    }
                    if o.default {
                        opt = opt.default_selection(true);
                    }
                    opt
                }).collect();
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

async fn call_runtime(code: String, context: RunContext) -> Option<RunResponse> {
    let client = reqwest::Client::new();
    let request = RunRequest { code, context };

    match client
        .post("http://localhost:3000/run")
        .json(&request)
        .send()
        .await
    {
        Ok(res) => res.json::<RunResponse>().await.ok(),
        Err(e) => {
            eprintln!("Failed to reach ZBR server: {}", e);
            None
        }
    }
}

/// Parse an emoji string into a serenity ReactionType.
/// Supports:
///   - Unicode emoji: "👍"
///   - Custom emoji format: "<:name:id>" or "<a:name:id>"
///   - Raw ID: "123456789"
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
