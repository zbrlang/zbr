use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// Zban{userIDs;reason?;deleteMessageDays?}
/// userIDs: single user ID or semicolon-separated list for bulk ban.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uids_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("ban", crate::error_messages::required(1, "userIDs")),
    };

    let user_ids: Vec<u64> = match uids_str
        .split(';')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(ids) => ids,
        Err(_) => return FnOutput::error("ban", crate::error_messages::expected_snowflake(1, "userIDs", &uids_str)),
    };

    if user_ids.is_empty() {
        return FnOutput::error("ban", crate::error_messages::required(1, "userIDs"));
    }

    let reason = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => String::new(),
    };

    let delete_days: u8 = match args.get(2) {
        Some(s) if !s.is_empty() => match s.parse::<u8>() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("ban", crate::error_messages::expected_integer(3, "deleteMessageDays", s)),
        },
        _ => 0,
    };

    if delete_days > 7 {
        return FnOutput::error("ban", crate::error_messages::out_of_range(3, "deleteMessageDays", 0, 7, delete_days as i64));
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("ban", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("ban", "no HTTP client available"),
    };

    let gid = GuildId::new(gid);
    let mut errors: Vec<String> = Vec::new();
    let mut banned: Vec<String> = Vec::new();

    let results: Vec<(String, bool)> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut results = Vec::new();
            for uid in &user_ids {
                let r = if reason.is_empty() {
                    gid.ban(&http, UserId::new(*uid), delete_days).await
                } else {
                    gid.ban_with_reason(&http, UserId::new(*uid), delete_days, &reason).await
                };
                match r {
                    Ok(_) => results.push((uid.to_string(), true)),
                    Err(_e) => results.push((uid.to_string(), false)),
                }
            }
            results
        })
    });

    for (id, ok) in results {
        if ok { banned.push(id); } else { errors.push(id); }
    }

    let mut parts = Vec::new();
    if !banned.is_empty() {
        parts.push(format!("banned: {}", banned.join(", ")));
    }
    if !errors.is_empty() {
        parts.push(format!("failed: {}", errors.join(", ")));
    }
    FnOutput::Text(parts.join(" | "))
}
