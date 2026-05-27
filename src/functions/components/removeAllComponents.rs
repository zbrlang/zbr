use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMessage;
use serenity::model::id::{ChannelId, MessageId};

/// ZremoveAllComponents{messageID?}
/// Removes all components from a message.
/// No arg = removes from the triggering message.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => match &ctx.trigger_message_id {
            Some(id) => id.clone(),
            None => return FnOutput::error("removeAllComponents", crate::error_messages::required(1, "messageID")),
        },
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeAllComponents", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeAllComponents", crate::error_messages::expected_snowflake(1, "messageID", &mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("removeAllComponents", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().components(vec![]))
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("removeAllComponents", crate::error_messages::action_failed("remove components")),
    }
}
