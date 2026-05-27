use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZrepliedMessageID{channelID?;messageID?}
/// No args = check the triggering message's referenced message.
/// With args = check a specific message.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    let mid_str = match args.get(1) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => ctx.trigger_message_id.clone(),
    };

    let mid_str = match mid_str {
        Some(s) => s,
        None => return FnOutput::Text(String::new()),
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("repliedMessageID", format!("invalid channel ID: '{}'", cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("repliedMessageID", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("repliedMessageID", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => {
            let id = msg.referenced_message
                .as_ref()
                .map(|r| r.id.to_string())
                .unwrap_or_default();
            FnOutput::Text(id)
        }
        Err(_) => FnOutput::error("repliedMessageID", crate::error_messages::not_found("message", &mid_str)),
    }
}
