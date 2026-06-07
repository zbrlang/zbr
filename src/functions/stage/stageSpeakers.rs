use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZstageSpeakers{channelID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("stageSpeakers", crate::error_messages::required(1, "channelID")),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stageSpeakers", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stageSpeakers", crate::error_messages::not_in_guild()),
    };

    let cache = ctx.cache.clone();

    let result = tokio::task::block_in_place(|| {
        let guild = match cache.guild(GuildId::new(gid)) {
            Some(g) => g,
            None => return Err("guild not found in cache".to_string()),
        };

        let mut speakers = Vec::new();
        for (user_id, voice_state) in &guild.voice_states {
            if voice_state.channel_id == Some(serenity::model::id::ChannelId::new(cid)) {
                if !voice_state.suppress {
                    speakers.push(user_id.to_string());
                }
            }
        }
        Ok(speakers.join(","))
    });

    match result {
        Ok(s) => FnOutput::Text(s),
        Err(e) => FnOutput::error("stageSpeakers", e),
    }
}
