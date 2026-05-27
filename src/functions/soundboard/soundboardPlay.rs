use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId, SoundId};

/// ZsoundboardPlay{soundId;voiceChannelId}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sound_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardPlay", crate::error_messages::required(1, "soundId")),
    };
    let vcid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardPlay", crate::error_messages::required(2, "voiceChannelId")),
    };

    let sid: u64 = match sound_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardPlay", crate::error_messages::expected_snowflake(1, "soundId", &sound_id_str)),
    };
    let vcid: u64 = match vcid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardPlay", crate::error_messages::expected_snowflake(2, "voiceChannelId", &vcid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardPlay", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardPlay", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(vcid)
                .send_soundboard(&http, SoundId::new(sid), Some(GuildId::new(gid)))
                .await
                .map_err(|e| crate::error_messages::action_failed_reason("play soundboard sound", &format!("{}", e)))
        })
    });

    match result {
        Ok(_) => FnOutput::Text("true".to_string()),
        Err(e) => FnOutput::error("soundboardPlay", e),
    }
}
