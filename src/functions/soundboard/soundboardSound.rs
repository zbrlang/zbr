use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, SoundId};

/// ZsoundboardSound{soundId}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sound_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardSound", crate::error_messages::required(1, "soundId")),
    };

    let sid: u64 = match sound_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardSound", crate::error_messages::expected_snowflake(1, "soundId", &sound_id_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardSound", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardSound", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).get_soundboards(&http).await
        })
    });

    match result {
        Ok(sounds) => {
            let sound = sounds.iter().find(|s| s.id == SoundId::new(sid));
            match sound {
                Some(s) => FnOutput::Text(serde_json::to_string(s).unwrap_or_default()),
                None => FnOutput::error("soundboardSound", crate::error_messages::not_found("soundboard sound", &sound_id_str)),
            }
        }
        Err(e) => FnOutput::error("soundboardSound", format!("{}", e)),
    }
}
