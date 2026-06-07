use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZvoiceRegion{channelID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("voiceRegion", crate::error_messages::required(1, "channelID")),
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceRegion", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceRegion", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(channel) => {
            if let Some(gc) = channel.guild() {
                FnOutput::Text(gc.rtc_region.clone().unwrap_or_else(|| "automatic".to_string()))
            } else {
                FnOutput::error("voiceRegion", "not a guild channel")
            }
        }
        Err(_) => FnOutput::error("voiceRegion", crate::error_messages::not_found("channel", &cid_str)),
    }
}
