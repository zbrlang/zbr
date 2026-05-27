use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    if cid_str.is_empty() {
        return FnOutput::error("channelPosition", crate::error_messages::required(1, "channel ID"));
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelPosition", crate::error_messages::expected_snowflake(1, "channel ID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelPosition", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.guild() {
                FnOutput::Text(guild_channel.position.to_string())
            } else {
                FnOutput::Text("0".to_string())
            }
        }
        Err(_) => FnOutput::error("channelPosition", crate::error_messages::not_found("channel", &cid_str)),
    }
}
