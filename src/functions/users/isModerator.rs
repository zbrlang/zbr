use crate::context::{DiscordContext, FnOutput};
use crate::functions::permissions::helpers::member_guild_permissions;
use serenity::model::id::{GuildId, UserId};
use serenity::model::permissions::Permissions;

/// ZisModerator{userID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }
    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isModerator", "invalid userID"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isModerator", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isModerator", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(gid)
                .member(&http, UserId::new(uid))
                .await
                .map_err(|e| format!("failed to fetch member: {}", e))?;
            member_guild_permissions(&http, gid, &member).await
        })
    });
    match result {
        Ok(perms) => FnOutput::Text(perms.contains(Permissions::MODERATE_MEMBERS).to_string()),
        Err(_) => FnOutput::error("isModerator", "user not found"),
    }
}
