use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// Zkick{userID;reason?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("kick", "userID is required"),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("kick", format!("invalid user ID: '{}'", uid_str)),
    };

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

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).kick_with_reason(&http, UserId::new(uid), reason.unwrap_or("")).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("kick", "failed to kick user"),
    }
}
