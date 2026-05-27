use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{UserId, GuildId};
use serenity::builder::EditMember;

/// ZchangeNickname{nickname;userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let nickname = args.get(0).cloned().unwrap_or_default();
    if nickname.len() > 32 {
        return FnOutput::error("changeNickname", crate::error_messages::too_long(1, "nickname", 32, nickname.len()));
    }

    let mut user_id_str = args.get(1).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("changeNickname", crate::error_messages::expected_snowflake(2, "userID", &user_id_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("changeNickname", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("changeNickname", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut member = GuildId::new(gid).member(&http, UserId::new(uid)).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch member", &e.to_string()))?;
            
            let final_nick = nickname.replace("%username%", &member.user.name);
            if final_nick.len() > 32 {
                return Err(crate::error_messages::too_long(1, "nickname", 32, final_nick.len()));
            }

            member.edit(&http, EditMember::new().nickname(final_nick)).await
                .map_err(|e| crate::error_messages::action_failed_reason("change nickname", &e.to_string()))?;
            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("changeNickname", e),
    }
}
