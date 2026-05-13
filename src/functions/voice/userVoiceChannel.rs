use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZuserVoiceChannel{userID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error(
                "userVoiceChannel",
                format!("invalid user ID: '{}'", user_id_str),
            )
        }
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userVoiceChannel", "not in a guild"),
    };

    let channel_id = ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
        g.voice_states
            .get(&UserId::new(uid))
            .and_then(|vs| vs.channel_id)
    });

    match channel_id {
        Some(cid) => FnOutput::Text(cid.to_string()),
        None => FnOutput::Text(String::new()),
    }
}
