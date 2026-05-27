use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZgetAttachments{index?}
/// No index = all attachment URLs joined by newline.
/// With index = that specific attachment URL (1-based).
/// Reads from the triggering message via HTTP.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index: Option<usize> = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => Some(n),
            _ => return FnOutput::error("getAttachments", format!("invalid index: '{}' (must be 1 or greater)", s)),
        },
        _ => None,
    };

    let mid_str = match &ctx.trigger_message_id {
        Some(id) => id.clone(),
        None => return FnOutput::Text(String::new()),
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getAttachments", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getAttachments", "invalid message ID in context"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("getAttachments", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => {
            let urls: Vec<String> = msg.attachments.iter().map(|a| a.url.clone()).collect();
            match index {
                Some(i) => FnOutput::Text(urls.get(i - 1).cloned().unwrap_or_default()),
                None => FnOutput::Text(urls.join("\n")),
            }
        }
        Err(_) => FnOutput::error("getAttachments", crate::error_messages::action_failed("fetch message")),
    }
}
