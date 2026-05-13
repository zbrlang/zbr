use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZdisplayName{userID?;type?}
/// type: omit or "server" = server display name (default), "global" = Discord global name
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }
    let name_type = args.get(1).map(|s| s.as_str()).unwrap_or("server");

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("displayName", "invalid userID"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("displayName", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("displayName", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async move { GuildId::new(gid).member(&http, UserId::new(uid)).await })
    });
    match result {
        Ok(member) => {
            if name_type == "global" {
                let name = member
                    .user
                    .global_name
                    .clone()
                    .unwrap_or_else(|| member.user.name.clone());
                FnOutput::Text(name)
            } else {
                FnOutput::Text(member.display_name().to_string())
            }
        }
        Err(_) => FnOutput::error("displayName", "user not found"),
    }
}
