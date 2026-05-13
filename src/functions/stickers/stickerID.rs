use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZstickerID{name} — find a sticker ID by name (case-insensitive)
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("stickerID", "name is required"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerID", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stickerID", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .stickers(&http)
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(stickers) => {
            let lower = name.to_lowercase();
            match stickers.iter().find(|s| s.name.to_lowercase() == lower) {
                Some(s) => FnOutput::Text(s.id.to_string()),
                None => FnOutput::Text(String::new()),
            }
        }
        Err(e) => FnOutput::error("stickerID", e),
    }
}
