use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};
use chrono_tz::Tz;

/// ZuserJoined{userID?;format?}
/// Returns the date when the user joined the current guild.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    let format_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => "%Y-%m-%d".to_string(),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userJoined", format!("invalid user ID: '{}'", uid_str)),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userJoined", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userJoined", "no HTTP client available"),
    };

    let tz_str = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { ctx.timezone.lock().await.clone() })
    });
    let tz: Tz = tz_str.parse().unwrap_or(chrono_tz::Asia::Tokyo);

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .member(&http, UserId::new(uid))
                .await
                .map_err(|_| "user not found".to_string())
        })
    });

    match result {
        Ok(member) => match member.joined_at {
            Some(ts) => {
                let dt = ts.with_timezone(&tz);
                FnOutput::Text(dt.format(&format_str).to_string())
            }
            None => FnOutput::Text(String::new()),
        },
        Err(e) => FnOutput::error("userJoined", e),
    }
}
