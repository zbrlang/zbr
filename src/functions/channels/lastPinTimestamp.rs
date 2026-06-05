use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;
use chrono_tz::Tz;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("lastPinTimestamp", crate::error_messages::expected_snowflake(1, "channel ID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("lastPinTimestamp", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    let ts_opt = match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.clone().guild() {
                guild_channel.last_pin_timestamp
            } else if let Some(private) = channel.private() {
                private.last_pin_timestamp
            } else {
                None
            }
        }
        Err(_) => return FnOutput::error("lastPinTimestamp", crate::error_messages::not_found("channel", &cid_str)),
    };

    match ts_opt {
        Some(ts) => {
            let tz_str = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.timezone.lock().await.clone()
                })
            });
            let tz: Tz = tz_str.parse().unwrap_or(chrono_tz::Asia::Tokyo);
            let dt = ts.with_timezone(&tz);
            FnOutput::Text(dt.format("%Y-%m-%d").to_string())
        }
        None => FnOutput::Text("".to_string()),
    }
}
