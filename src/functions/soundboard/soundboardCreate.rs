use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateAttachment, CreateSoundboard};
use serenity::model::id::GuildId;

/// ZsoundboardCreate{name;fileUrl}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardCreate", crate::error_messages::required(1, "name")),
    };
    let file_url = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("soundboardCreate", crate::error_messages::required(2, "fileUrl")),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardCreate", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardCreate", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let attachment = match CreateAttachment::url(&http, &file_url).await {
                Ok(a) => a,
                Err(e) => return Err(crate::error_messages::action_failed_reason("download file", &format!("{}", e))),
            };
            let builder = CreateSoundboard::new(&name, &attachment);
            GuildId::new(gid)
                .create_soundboard(&http, builder)
                .await
                .map(|s| s.id.to_string())
                .map_err(|e| crate::error_messages::action_failed_reason("create soundboard sound", &format!("{}", e)))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("soundboardCreate", e),
    }
}
