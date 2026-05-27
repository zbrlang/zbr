use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{UserId, GuildId};

/// ZuserServerAvatar{userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userServerAvatar", crate::error_messages::expected_snowflake(1, "userID", &user_id_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userServerAvatar", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userServerAvatar", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).member(&http, UserId::new(uid)).await
        })
    });

    match result {
        Ok(member) => {
            let url = member.avatar_url().unwrap_or_else(|| {
                member.user.avatar_url().unwrap_or_else(|| member.user.default_avatar_url())
            });
            FnOutput::Text(url)
        }
        Err(_) => FnOutput::error("userServerAvatar", crate::error_messages::not_found("user", &user_id_str)),
    }
}
