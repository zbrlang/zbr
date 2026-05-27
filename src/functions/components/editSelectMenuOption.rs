use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{EditMessage, CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};
use serenity::model::id::{ChannelId, MessageId};

/// ZeditSelectMenuOption{menuID;label;value;description?;default?;emoji?;messageID?}
/// Replaces the options on an existing select menu with a single updated option.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let menu_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editSelectMenuOption", crate::error_messages::required(1, "menuID")),
    };
    let label = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editSelectMenuOption", crate::error_messages::required(2, "label")),
    };
    let value = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editSelectMenuOption", crate::error_messages::required(3, "value")),
    };
    let description = match args.get(3) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };
    let default = match args.get(4) {
        Some(s) if s == "true" => true,
        _ => false,
    };
    let mid_str = match args.get(6) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => match &ctx.trigger_message_id {
            Some(id) => id.clone(),
            None => return FnOutput::error("editSelectMenuOption", crate::error_messages::required(7, "messageID")),
        },
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editSelectMenuOption", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editSelectMenuOption", crate::error_messages::expected_snowflake(7, "message ID", &mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editSelectMenuOption", "no HTTP client available"),
    };

    let mut opt = CreateSelectMenuOption::new(&label, &value);
    if let Some(desc) = description {
        opt = opt.description(desc);
    }
    if default {
        opt = opt.default_selection(true);
    }

    let menu = CreateSelectMenu::new(&menu_id, CreateSelectMenuKind::String { options: vec![opt] });

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().components(vec![
                    CreateActionRow::SelectMenu(menu),
                ]))
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("editSelectMenuOption", crate::error_messages::action_failed("edit select menu option")),
    }
}
