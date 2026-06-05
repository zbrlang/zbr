use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZgetMessage{channelID;messageID;type?}
/// type: content (default), authorID, username, avatar
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or(ctx.channel_id.clone());
    let mid_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let msg_type = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or("content".to_string());

    if mid_str.is_empty() {
        return FnOutput::error("getMessage", "messageID is required");
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getMessage", format!("invalid channel ID: '{}'", cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getMessage", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("getMessage", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => match msg_type.as_str() {
            "content" => FnOutput::Text(msg.content.clone()),
            "authorID" => FnOutput::Text(msg.author.id.to_string()),
            "username" => FnOutput::Text(msg.author.name.clone()),
            "avatar" => FnOutput::Text(
                msg.author
                    .avatar_url()
                    .unwrap_or_else(|| msg.author.default_avatar_url()),
            ),
            other => FnOutput::error(
                "getMessage",
                crate::error_messages::expected_choice(3, "type", "content, authorID, username, or avatar", other),
            ),
        },
        Err(_) => FnOutput::error("getMessage", crate::error_messages::not_found("message", &mid_str)),
    }
}
