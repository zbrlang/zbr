use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    if cid_str.is_empty() {
        return FnOutput::error("channelTopic", "invalid channel ID");
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelTopic", "invalid channel ID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelTopic", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.guild() {
                FnOutput::Text(guild_channel.topic.unwrap_or_default())
            } else {
                FnOutput::Text("".to_string())
            }
        }
        Err(_) => FnOutput::error("channelTopic", "channel not found"),
    }
}
