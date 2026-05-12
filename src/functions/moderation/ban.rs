use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// Zban{userID;reason?;deleteMessageDays?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("ban", "userID is required"),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("ban", format!("invalid user ID: '{}'", uid_str)),
    };

    let reason = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => String::new(),
    };

    let delete_days: u8 = match args.get(2) {
        Some(s) if !s.is_empty() => match s.parse::<u8>() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("ban", "deleteMessageDays must be between 0 and 7"),
        },
        _ => 0,
    };

    if delete_days > 7 {
        return FnOutput::error("ban", "deleteMessageDays must be between 0 and 7");
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("ban", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("ban", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            if reason.is_empty() {
                GuildId::new(gid).ban(&http, UserId::new(uid), delete_days).await
            } else {
                GuildId::new(gid)
                    .ban_with_reason(&http, UserId::new(uid), delete_days, &reason)
                    .await
            }
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("ban", "failed to ban user"),
    }
}
