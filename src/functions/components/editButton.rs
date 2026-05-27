use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{EditMessage, CreateActionRow, CreateButton};
use serenity::model::application::ButtonStyle;
use serenity::model::id::{ChannelId, MessageId};

/// ZeditButton{customID;label;style;disabled?;emoji?;messageID?}
/// Edits a button on an existing message by replacing the entire component row.
/// messageID defaults to the triggering message.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let custom_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editButton", crate::error_messages::required(1, "customID")),
    };
    let label = args.get(1).cloned().unwrap_or_default();
    let style_str = match args.get(2) {
        Some(s) if !s.is_empty() => s.to_lowercase(),
        _ => "secondary".to_string(),
    };
    let disabled = match args.get(3) {
        Some(s) if s == "true" => true,
        _ => false,
    };
    let mid_str = match args.get(5) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => match &ctx.trigger_message_id {
            Some(id) => id.clone(),
            None => return FnOutput::error("editButton", crate::error_messages::required(6, "messageID")),
        },
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editButton", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editButton", crate::error_messages::expected_snowflake(6, "message ID", &mid_str)),
    };

    let style = match style_str.as_str() {
        "primary"   => ButtonStyle::Primary,
        "success"   => ButtonStyle::Success,
        "danger"    => ButtonStyle::Danger,
        _           => ButtonStyle::Secondary,
    };

    let mut btn = if style_str == "link" {
        CreateButton::new_link(&custom_id)
    } else {
        CreateButton::new(&custom_id).style(style)
    };
    btn = btn.label(&label).disabled(disabled);

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editButton", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().components(vec![
                    CreateActionRow::Buttons(vec![btn]),
                ]))
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("editButton", crate::error_messages::action_failed("edit button")),
    }
}
