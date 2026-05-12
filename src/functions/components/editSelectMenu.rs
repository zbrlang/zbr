use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{EditMessage, CreateActionRow, CreateSelectMenu, CreateSelectMenuKind};
use serenity::model::id::{ChannelId, MessageId};

/// ZeditSelectMenu{menuID;min;max;placeholder?;messageID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let menu_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editSelectMenu", "menuID is required"),
    };
    let min: u8 = match args.get(1) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("editSelectMenu", format!("invalid min: '{}'", s)),
        },
        _ => 1,
    };
    let max: u8 = match args.get(2) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("editSelectMenu", format!("invalid max: '{}'", s)),
        },
        _ => 1,
    };
    let placeholder = match args.get(3) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };
    let mid_str = match args.get(4) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => match &ctx.trigger_message_id {
            Some(id) => id.clone(),
            None => return FnOutput::error("editSelectMenu", "messageID is required"),
        },
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editSelectMenu", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editSelectMenu", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editSelectMenu", "no HTTP client available"),
    };

    let mut menu = CreateSelectMenu::new(&menu_id, CreateSelectMenuKind::String { options: vec![] })
        .min_values(min)
        .max_values(max);
    if let Some(ph) = placeholder {
        menu = menu.placeholder(ph);
    }

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
        Err(_) => FnOutput::error("editSelectMenu", "failed to edit select menu"),
    }
}
