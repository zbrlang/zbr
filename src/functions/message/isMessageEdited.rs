use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZisMessageEdited{channelID;messageID;type?}
/// type: edited (default), timestamp
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or(ctx.channel_id.clone());
    let mid_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let out_type = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or("edited".to_string());

    if mid_str.is_empty() {
        return FnOutput::error("isMessageEdited", "messageID is required");
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isMessageEdited", format!("invalid channel ID: '{}'", cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isMessageEdited", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isMessageEdited", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => match out_type.as_str() {
            "edited" => FnOutput::Text(msg.edited_timestamp.is_some().to_string()),
            "timestamp" => FnOutput::Text(
                msg.edited_timestamp
                    .map(|t| t.to_string())
                    .unwrap_or_default(),
            ),
            other => FnOutput::error(
                "isMessageEdited",
                crate::error_messages::expected_choice(3, "type", "edited, timestamp", other),
            ),
        },
        Err(_) => FnOutput::error("isMessageEdited", crate::error_messages::not_found("message", &mid_str)),
    }
}
