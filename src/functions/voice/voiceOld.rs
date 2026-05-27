use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZvoiceOld{}
/// Returns the previous voice channel ID from the onVoiceStateUpdate event context
/// (the channel the user was in before).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceOld", crate::error_messages::not_in_guild()),
    };

    let author_uid: u64 = match ctx.author_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceOld", crate::error_messages::expected_snowflake(1, "authorID", &ctx.author_id)),
    };

    let channel_id = ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
        g.voice_states
            .get(&UserId::new(author_uid))
            .and_then(|vs| vs.channel_id)
    });

    match channel_id {
        Some(cid) => FnOutput::Text(cid.to_string()),
        None => FnOutput::Text(String::new()),
    }
}
