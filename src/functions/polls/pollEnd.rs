use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollEnd", "messageID is required"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollEnd", format!("invalid message ID: '{}'", mid_str)),
    };
    let cid_str = args.get(1).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollEnd", format!("invalid channel ID: '{}'", cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("pollEnd", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).end_poll(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("pollEnd", e.to_string()),
    }
}
