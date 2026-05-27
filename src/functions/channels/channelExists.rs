use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    if cid_str.is_empty() {
        return FnOutput::error("channelExists", crate::error_messages::required(1, "channel ID"));
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::Text("false".to_string()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelExists", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(_) => FnOutput::Text("true".to_string()),
        Err(_) => FnOutput::Text("false".to_string()),
    }
}
