use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZisTimedOut{userID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("isTimedOut", format!("invalid user ID: '{}'", uid_str))
        }
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isTimedOut", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isTimedOut", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_member(GuildId::new(gid), UserId::new(uid)).await
        })
    });

    match result {
        Ok(member) => {
            let timed_out = member
                .communication_disabled_until
                .map(|ts| ts.unix_timestamp() > chrono::Utc::now().timestamp())
                .unwrap_or(false);
            FnOutput::Text(timed_out.to_string())
        }
        Err(_) => FnOutput::error("isTimedOut", "user not found"),
    }
}
