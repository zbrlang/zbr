use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZisBanned{userID;guildID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("isBanned", "userID is required"),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isBanned", format!("invalid user ID: '{}'", uid_str)),
    };

    let gid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.guild_id.clone(),
    };

    let gid: u64 = match gid_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("isBanned", format!("invalid guild ID: '{}'", gid_str))
        }
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isBanned", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).get_ban(&http, UserId::new(uid)).await
        })
    });

    match result {
        Ok(Some(_)) => FnOutput::Text("true".to_string()),
        Ok(None) | Err(_) => FnOutput::Text("false".to_string()),
    }
}
