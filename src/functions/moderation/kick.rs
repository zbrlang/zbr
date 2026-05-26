use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// Zkick{userIDs;reason?}
/// userIDs: single user ID or semicolon-separated list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uids_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("kick", "userIDs is required"),
    };

    let user_ids: Vec<u64> = match uids_str
        .split(';')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(ids) => ids,
        Err(_) => return FnOutput::error("kick", format!("invalid user ID: '{}'", uids_str)),
    };

    if user_ids.is_empty() {
        return FnOutput::error("kick", "userIDs is required");
    }

    let reason = match args.get(1) {
        Some(s) if !s.is_empty() => Some(s.as_str()),
        _ => None,
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("kick", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("kick", "no HTTP client available"),
    };

    let gid = GuildId::new(gid);
    let mut kicked: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let results: Vec<(String, bool)> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut results = Vec::new();
            for uid in &user_ids {
                match gid.kick_with_reason(&http, UserId::new(*uid), reason.unwrap_or("")).await {
                    Ok(_) => results.push((uid.to_string(), true)),
                    Err(_) => results.push((uid.to_string(), false)),
                }
            }
            results
        })
    });

    for (id, ok) in results {
        if ok { kicked.push(id); } else { errors.push(id); }
    }

    let mut parts = Vec::new();
    if !kicked.is_empty() {
        parts.push(format!("kicked: {}", kicked.join(", ")));
    }
    if !errors.is_empty() {
        parts.push(format!("failed: {}", errors.join(", ")));
    }
    FnOutput::Text(parts.join(" | "))
}
