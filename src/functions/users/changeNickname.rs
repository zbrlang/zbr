use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{UserId, GuildId};
use serenity::builder::EditMember;

/// ZchangeNickname{nickname;userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let nickname = args.get(0).cloned().unwrap_or_default();
    if nickname.len() > 32 {
        return FnOutput::error("changeNickname", "nickname cannot exceed 32 characters");
    }

    let mut user_id_str = args.get(1).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("changeNickname", "invalid userID"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("changeNickname", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("changeNickname", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut member = GuildId::new(gid).member(&http, UserId::new(uid)).await
                .map_err(|e| format!("failed to fetch member: {}", e))?;
            
            let final_nick = nickname.replace("%username%", &member.user.name);
            if final_nick.len() > 32 {
                return Err("nickname cannot exceed 32 characters after replacing %username%".to_string());
            }

            member.edit(&http, EditMember::new().nickname(final_nick)).await
                .map_err(|e| format!("failed to change nickname: {}", e))?;
            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("changeNickname", e),
    }
}
