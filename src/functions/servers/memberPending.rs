use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("memberPending", crate::error_messages::expected_snowflake(1, "userID", &uid_str))
        }
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("memberPending", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("memberPending", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_member(GuildId::new(gid), UserId::new(uid)).await
        })
    });

    match result {
        Ok(member) => FnOutput::Text(member.pending.to_string()),
        Err(_) => FnOutput::error("memberPending", crate::error_messages::not_found("member", &uid_str)),
    }
}
