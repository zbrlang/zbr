use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMember;
use serenity::model::id::{GuildId, UserId};

/// ZvoiceKick{userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("voiceKick", crate::error_messages::required(1, "userID")),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceKick", crate::error_messages::expected_snowflake(1, "userID", &uid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceKick", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceKick", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditMember::new().disconnect_member();
            GuildId::new(gid)
                .edit_member(&http, UserId::new(uid), builder)
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error(
            "voiceKick",
            crate::error_messages::action_failed_reason("disconnect user", "are they in a voice channel?"),
        ),
    }
}
