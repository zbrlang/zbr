use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, SoundId};

/// ZsoundboardDelete{soundId}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sound_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardDelete", "soundId is required"),
    };

    let sid: u64 = match sound_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardDelete", format!("invalid soundId: '{}'", sound_id_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardDelete", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardDelete", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .delete_soundboard(&http, SoundId::new(sid), None)
                .await
                .map_err(|e| format!("failed to delete soundboard sound: {}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Text("true".to_string()),
        Err(e) => FnOutput::error("soundboardDelete", e),
    }
}
