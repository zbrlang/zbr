use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZisTimedOut{userID?;returnTimestamp?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    let return_timestamp = match args.get(1) {
        Some(s) => s.to_lowercase() == "true",
        None => false,
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("isTimedOut", crate::error_messages::expected_snowflake(1, "userID", &uid_str))
        }
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isTimedOut", crate::error_messages::not_in_guild()),
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
            if let Some(ts) = member.communication_disabled_until {
                let expiry = ts.unix_timestamp();
                if expiry > chrono::Utc::now().timestamp() {
                    if return_timestamp {
                        return FnOutput::Text(expiry.to_string());
                    } else {
                        return FnOutput::Text("true".to_string());
                    }
                }
            }
            if return_timestamp {
                FnOutput::Text(String::new())
            } else {
                FnOutput::Text("false".to_string())
            }
        }
        Err(_) => FnOutput::error("isTimedOut", crate::error_messages::not_found("member", &uid_str)),
    }
}
