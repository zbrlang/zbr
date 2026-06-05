use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;
use serenity::builder::EditChannel;
use crate::functions::cooldown::helpers::parse_duration;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    if cid_str.is_empty() {
        return FnOutput::error("slowMode", crate::error_messages::required(1, "channel ID"));
    }
    let dur_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if dur_str.is_empty() {
        return FnOutput::error("slowMode", crate::error_messages::required(2, "duration"));
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("slowMode", crate::error_messages::expected_snowflake(1, "channel ID", &cid_str)),
    };

    // "0" or "0s" disables slowmode
    let secs: u64 = if dur_str == "0" || dur_str == "0s" {
        0
    } else if let Ok(n) = dur_str.parse::<u64>() {
        // plain integer → raw seconds
        n
    } else {
        // duration string like "30s", "1m30s", etc.
        match parse_duration(&dur_str) {
            Ok(s) => s as u64,
            Err(_) => return FnOutput::error("slowMode", crate::error_messages::expected_duration(2, "duration", &dur_str)),
        }
    };

    if secs > 21600 {
        return FnOutput::error("slowMode", "slowmode cannot exceed 6 hours (21600 seconds)");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("slowMode", "no HTTP client available"),
    };

    let builder = EditChannel::new().rate_limit_per_user(secs as u16);

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).edit(&http, builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("slowMode", crate::error_messages::not_found("channel", &cid_str)),
    }
}
