use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId, SoundId};

/// ZsoundboardPlay{soundId;voiceChannelId}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sound_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardPlay", "soundId is required"),
    };
    let vcid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardPlay", "voiceChannelId is required"),
    };

    let sid: u64 = match sound_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardPlay", format!("invalid soundId: '{}'", sound_id_str)),
    };
    let vcid: u64 = match vcid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardPlay", format!("invalid voiceChannelId: '{}'", vcid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardPlay", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardPlay", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(vcid)
                .send_soundboard(&http, SoundId::new(sid), Some(GuildId::new(gid)))
                .await
                .map_err(|e| format!("failed to play soundboard sound: {}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Text("true".to_string()),
        Err(e) => FnOutput::error("soundboardPlay", e),
    }
}
