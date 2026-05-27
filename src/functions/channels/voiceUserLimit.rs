use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;
use serenity::model::channel::ChannelType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceUserLimit", crate::error_messages::expected_snowflake(1, "channel ID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceUserLimit", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.guild() {
                if guild_channel.kind != ChannelType::Voice && guild_channel.kind != ChannelType::Stage {
                    return FnOutput::error("voiceUserLimit", "channel is not a voice or stage channel");
                }
                FnOutput::Text(guild_channel.user_limit.unwrap_or(0).to_string())
            } else {
                FnOutput::error("voiceUserLimit", "channel is not a voice or stage channel")
            }
        }
        Err(_) => FnOutput::error("voiceUserLimit", crate::error_messages::not_found("channel", &cid_str)),
    }
}
