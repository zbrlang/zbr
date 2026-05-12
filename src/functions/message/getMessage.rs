use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZgetMessage{channelID;messageID;type?}
/// type: content (default), authorID, username, avatar
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("getMessage", "channelID is required"),
    };
    let mid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("getMessage", "messageID is required"),
    };
    let msg_type = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => "content".to_string(),
    };

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
                format!("invalid type: '{}' (expected content, authorID, username, or avatar)", other),
            ),
        },
        Err(_) => FnOutput::error("getMessage", "message not found"),
    }
}
