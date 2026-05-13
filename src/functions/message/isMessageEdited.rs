use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZisMessageEdited{channelID;messageID;type?}
/// type: edited (default), timestamp
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let (cid_str, mid_str, out_type) = match args.len() {
        0 => (ctx.channel_id.clone(), String::new(), "edited".to_string()),
        1 => (ctx.channel_id.clone(), args[0].clone(), "edited".to_string()),
        _ => {
            let cid_str = match args.get(0) {
                Some(s) if !s.is_empty() => s.clone(),
                _ => ctx.channel_id.clone(),
            };
            (
                cid_str,
                args.get(1).cloned().unwrap_or_default(),
                match args.get(2) {
                    Some(s) if !s.is_empty() => s.clone(),
                    _ => "edited".to_string(),
                },
            )
        }
    };
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
                format!("invalid type: '{}' (expected edited or timestamp)", other),
            ),
        },
        Err(_) => FnOutput::error("isMessageEdited", "message not found"),
    }
}
