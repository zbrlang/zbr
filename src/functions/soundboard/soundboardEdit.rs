use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditSoundboard;
use serenity::model::id::{GuildId, SoundId};

/// ZsoundboardEdit{soundId;name;volume;emoji}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sound_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardEdit", crate::error_messages::required(1, "soundId")),
    };

    let sid: u64 = match sound_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardEdit", crate::error_messages::expected_snowflake(1, "soundId", &sound_id_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardEdit", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardEdit", crate::error_messages::action_failed("get HTTP client")),
    };

    let name = args.get(1).filter(|s| !s.is_empty()).cloned();
    let volume = args.get(2).and_then(|s| {
        if s.is_empty() { None } else { s.parse::<f64>().ok() }
    });
    let emoji = args.get(3).filter(|s| !s.is_empty()).cloned();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditSoundboard::new();
            if let Some(n) = name {
                builder = builder.name(n);
            }
            if let Some(v) = volume {
                builder = builder.volume(v);
            }
            if let Some(e) = emoji {
                builder = builder.emoji_name(e);
            }
            GuildId::new(gid)
                .edit_soundboard(&http, SoundId::new(sid), builder)
                .await
                .map(|s| s.id.to_string())
                .map_err(|e| crate::error_messages::action_failed_reason("edit soundboard sound", &format!("{}", e)))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("soundboardEdit", e),
    }
}
