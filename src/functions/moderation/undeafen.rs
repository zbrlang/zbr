use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMember;
use serenity::model::id::{GuildId, UserId};

/// Zundeafen{userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("undeafen", "userID is required"),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("undeafen", format!("invalid user ID: '{}'", uid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("undeafen", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("undeafen", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditMember::new().deafen(false);
            GuildId::new(gid)
                .edit_member(&http, UserId::new(uid), builder)
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("undeafen", "failed to server-undeafen user"),
    }
}
