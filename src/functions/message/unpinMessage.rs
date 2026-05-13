use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZunpinMessage{channelID;messageID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let (cid_str, mid_str) = match args.len() {
        0 => (ctx.channel_id.clone(), String::new()),
        1 => (ctx.channel_id.clone(), args[0].clone()),
        _ => {
            let cid_str = match args.get(0) {
                Some(s) if !s.is_empty() => s.clone(),
                _ => ctx.channel_id.clone(),
            };
            (cid_str, args.get(1).cloned().unwrap_or_default())
        }
    };
    if mid_str.is_empty() {
        return FnOutput::error("unpinMessage", "messageID is required");
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("unpinMessage", format!("invalid channel ID: '{}'", cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("unpinMessage", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("unpinMessage", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).unpin(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("unpinMessage", "failed to unpin message"),
    }
}
