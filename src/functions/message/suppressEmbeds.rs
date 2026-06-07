use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMessage;
use serenity::model::id::{ChannelId, MessageId};

/// ZsuppressEmbeds{messageID;boolean}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("suppressEmbeds", crate::error_messages::required(1, "messageID")),
    };
    let suppress = match args.get(1) {
        Some(s) => s.to_lowercase() == "true",
        None => return FnOutput::error("suppressEmbeds", crate::error_messages::required(2, "boolean")),
    };

    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("suppressEmbeds", crate::error_messages::expected_snowflake(1, "messageID", &mid_str)),
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("suppressEmbeds", "invalid channel ID in context"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("suppressEmbeds", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut flags = serenity::model::channel::MessageFlags::empty();
            if suppress {
                flags |= serenity::model::channel::MessageFlags::SUPPRESS_EMBEDS;
            }
            
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().flags(flags))
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("suppressEmbeds", format!("failed to edit message flags: {}", e)),
    }
}
