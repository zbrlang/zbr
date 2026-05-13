use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZuserServerMuted{userID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error(
                "userServerMuted",
                format!("invalid user ID: '{}'", user_id_str),
            )
        }
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userServerMuted", "not in a guild"),
    };

    let val = ctx
        .cache
        .guild(GuildId::new(gid))
        .and_then(|g| g.voice_states.get(&UserId::new(uid)).map(|vs| vs.mute))
        .unwrap_or(false);

    FnOutput::Text(val.to_string())
}
