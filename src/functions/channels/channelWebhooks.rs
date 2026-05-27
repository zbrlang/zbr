use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZchannelWebhooks{channelID?}
/// Returns a space-separated list of webhook URLs for the given channel.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let channel_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    let cid: u64 = match channel_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error(
                "channelWebhooks",
                crate::error_messages::expected_snowflake(1, "channel ID", &channel_id_str),
            )
        }
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelWebhooks", crate::error_messages::requires_set_first("HTTP client")),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .webhooks(&http)
                .await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch webhooks", &e.to_string()))
        })
    });
    match result {
        Ok(hooks) => {
            let urls: Vec<String> = hooks.into_iter().filter_map(|w| w.url().ok()).collect();
            FnOutput::Text(urls.join(" "))
        }
        Err(e) => FnOutput::error("channelWebhooks", e),
    }
}
