use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZmessageFlags{messageID;flagName}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("messageFlags", crate::error_messages::required(1, "messageID")),
    };
    let flag_name = match args.get(1) {
        Some(s) if !s.is_empty() => s.to_uppercase(),
        _ => return FnOutput::error("messageFlags", crate::error_messages::required(2, "flagName")),
    };

    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("messageFlags", crate::error_messages::expected_snowflake(1, "messageID", &mid_str)),
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("messageFlags", crate::error_messages::internal_error("invalid channel ID in context")),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("messageFlags", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => {
            let flags = msg.flags.unwrap_or_default();
            let has_flag = match flag_name.as_str() {
                "SUPPRESS_EMBEDS" => flags.contains(serenity::model::channel::MessageFlags::SUPPRESS_EMBEDS),
                "EPHEMERAL" => flags.contains(serenity::model::channel::MessageFlags::EPHEMERAL),
                "HAS_THREAD" => flags.contains(serenity::model::channel::MessageFlags::HAS_THREAD),
                "LOADING" => flags.contains(serenity::model::channel::MessageFlags::LOADING),
                "SUPPRESS_NOTIFICATIONS" => flags.contains(serenity::model::channel::MessageFlags::SUPPRESS_NOTIFICATIONS),
                _ => return FnOutput::error("messageFlags", format!("invalid flag: '{}'", flag_name)),
            };
            FnOutput::Text(has_flag.to_string())
        }
        Err(_) => FnOutput::error("messageFlags", crate::error_messages::not_found("message", &mid_str)),
    }
}
