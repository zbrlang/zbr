use crate::context::{DiscordContext, FnOutput};
use crate::functions::cooldown::helpers::parse_duration;
use chrono::Utc;
use serenity::builder::EditMember;
use serenity::model::id::{GuildId, UserId};
use serenity::model::Timestamp;

const MAX_TIMEOUT_SECS: i64 = 28 * 24 * 3600; // 28 days

/// Ztimeout{userIDs;duration}
/// userIDs: single user ID or semicolon-separated list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uids_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("timeout", crate::error_messages::required(1, "userIDs")),
    };

    let dur_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("timeout", crate::error_messages::required(2, "duration")),
    };

    let user_ids: Vec<u64> = match uids_str
        .split(';')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(ids) => ids,
        Err(_) => return FnOutput::error("timeout", crate::error_messages::expected_snowflake(1, "userIDs", &uids_str)),
    };

    if user_ids.is_empty() {
        return FnOutput::error("timeout", crate::error_messages::required(1, "userIDs"));
    }

    let secs = match parse_duration(&dur_str) {
        Ok(s) => s,
        Err(_) => return FnOutput::error("timeout", crate::error_messages::expected_duration(2, "duration", &dur_str)),
    };

    if secs > MAX_TIMEOUT_SECS {
        return FnOutput::error("timeout", "timeout cannot exceed 28 days");
    }

    let until = Utc::now() + chrono::Duration::seconds(secs);
    let timestamp = match Timestamp::from_unix_timestamp(until.timestamp()) {
        Ok(ts) => ts,
        Err(_) => return FnOutput::error("timeout", crate::error_messages::action_failed("compute timeout timestamp")),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("timeout", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("timeout", "no HTTP client available"),
    };

    let gid = GuildId::new(gid);
    let mut timed_out: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let results: Vec<(String, bool)> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut results = Vec::new();
            let builder = EditMember::new().disable_communication_until_datetime(timestamp);
            for uid in &user_ids {
                match gid.edit_member(&http, UserId::new(*uid), builder.clone()).await {
                    Ok(_) => results.push((uid.to_string(), true)),
                    Err(_) => results.push((uid.to_string(), false)),
                }
            }
            results
        })
    });

    for (id, ok) in results {
        if ok { timed_out.push(id); } else { errors.push(id); }
    }

    let mut parts = Vec::new();
    if !timed_out.is_empty() {
        parts.push(format!("timed out: {}", timed_out.join(", ")));
    }
    if !errors.is_empty() {
        parts.push(format!("failed: {}", errors.join(", ")));
    }
    FnOutput::Text(parts.join(" | "))
}
