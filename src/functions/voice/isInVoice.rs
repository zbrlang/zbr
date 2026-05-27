use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZisInVoice{userID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("isInVoice", crate::error_messages::expected_snowflake(1, "user ID", &user_id_str))
        }
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isInVoice", crate::error_messages::not_in_guild()),
    };

    let in_voice = ctx
        .cache
        .guild(GuildId::new(gid))
        .and_then(|g| {
            g.voice_states
                .get(&UserId::new(uid))
                .and_then(|vs| vs.channel_id)
        })
        .is_some();

    FnOutput::Text(in_voice.to_string())
}
