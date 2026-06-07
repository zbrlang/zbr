use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZforwardMessage{sourceMessageID;targetChannelID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forwardMessage", crate::error_messages::required(1, "sourceMessageID")),
    };
    let target_cid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forwardMessage", crate::error_messages::required(2, "targetChannelID")),
    };

    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forwardMessage", crate::error_messages::expected_snowflake(1, "sourceMessageID", &mid_str)),
    };
    let target_cid: u64 = match target_cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forwardMessage", crate::error_messages::expected_snowflake(2, "targetChannelID", &target_cid_str)),
    };

    let source_cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forwardMessage", "invalid source channel ID in context"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forwardMessage", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            // Discord's forward feature is often implemented as a specialized POST to messages endpoint with a snapshot.
            // In serenity 0.12, this might be a manual request if not directly exposed in the high-level API yet.
            // However, the user specifically mentioned "using Discord's message forward feature".
            // Since I cannot find a direct 'forward' method in serenity 0.12 docs for Message/ChannelId, 
            // I'll check if it's available or if I should use a generic request.
            // Actually, many wrappers just use the 'snapshot' field in CreateMessage.
            use serenity::builder::CreateMessage;
            use serenity::model::channel::{MessageReference, MessageReferenceKind};

            // Using the tuple From implementation
            let reference = MessageReference::from((ChannelId::new(source_cid), MessageId::new(mid)));

            ChannelId::new(target_cid).send_message(&http, CreateMessage::new().reference_message(reference)).await


        })
    });

    match result {
        Ok(m) => FnOutput::Text(m.id.to_string()),
        Err(e) => FnOutput::error("forwardMessage", format!("failed to forward message: {}", e)),
    }
}
