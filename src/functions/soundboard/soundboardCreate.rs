use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateAttachment, CreateSoundboard};
use serenity::model::id::GuildId;

/// ZsoundboardCreate{name;fileUrl}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardCreate", "name is required"),
    };
    let file_url = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardCreate", "fileUrl is required"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardCreate", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardCreate", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let attachment = match CreateAttachment::url(&http, &file_url).await {
                Ok(a) => a,
                Err(e) => return Err(format!("failed to download file: {}", e)),
            };
            let builder = CreateSoundboard::new(&name, &attachment);
            GuildId::new(gid)
                .create_soundboard(&http, builder)
                .await
                .map(|s| s.id.to_string())
                .map_err(|e| format!("failed to create soundboard sound: {}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("soundboardCreate", e),
    }
}
