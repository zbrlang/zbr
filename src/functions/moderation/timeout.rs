use crate::context::{DiscordContext, FnOutput};
use crate::functions::cooldown::helpers::parse_duration;
use chrono::Utc;
use serenity::builder::EditMember;
use serenity::model::id::{GuildId, UserId};
use serenity::model::Timestamp;

const MAX_TIMEOUT_SECS: i64 = 28 * 24 * 3600; // 28 days

/// Ztimeout{userID;duration}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("timeout", "userID is required"),
    };

    let dur_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("timeout", "duration is required"),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("timeout", format!("invalid user ID: '{}'", uid_str)),
    };

    let secs = match parse_duration(&dur_str) {
        Ok(s) => s,
        Err(_) => return FnOutput::error("timeout", format!("invalid duration: '{}'", dur_str)),
    };

    if secs > MAX_TIMEOUT_SECS {
        return FnOutput::error("timeout", "timeout cannot exceed 28 days");
    }

    let until = Utc::now() + chrono::Duration::seconds(secs);
    let timestamp = match Timestamp::from_unix_timestamp(until.timestamp()) {
        Ok(ts) => ts,
        Err(_) => return FnOutput::error("timeout", "failed to compute timeout timestamp"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("timeout", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("timeout", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditMember::new().disable_communication_until_datetime(timestamp);
            GuildId::new(gid)
                .edit_member(&http, UserId::new(uid), builder)
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("timeout", "failed to timeout user"),
    }
}
