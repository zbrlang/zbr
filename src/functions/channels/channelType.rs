use crate::context::{DiscordContext, FnOutput};
use serenity::model::channel::ChannelType;
use serenity::model::id::ChannelId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelType", "invalid channel ID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelType", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async move { ChannelId::new(cid).to_channel(&http).await })
    });

    match result {
        Ok(channel) => {
            let kind = match channel {
                serenity::model::channel::Channel::Guild(g) => g.kind,
                serenity::model::channel::Channel::Private(_) => {
                    return FnOutput::Text("dm".to_string())
                }
                _ => return FnOutput::Text("unknown".to_string()),
            };

            let type_str = match kind {
                ChannelType::Text => "text",
                ChannelType::Voice => "voice",
                ChannelType::Category => "category",
                ChannelType::News => "announcement",
                ChannelType::Stage => "stage",
                ChannelType::Forum => "forum",
                ChannelType::PublicThread
                | ChannelType::PrivateThread
                | ChannelType::NewsThread => "thread",
                _ => "unknown",
            };
            FnOutput::Text(type_str.to_string())
        }
        Err(_) => FnOutput::error("channelType", "channel not found"),
    }
}
