use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, StickerId};

/// ZstickerEmoji{stickerID} — returns the related emoji tag (e.g. "👍")
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("stickerEmoji", "stickerID is required"),
    };
    let sid: u64 = match sid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerEmoji", "invalid sticker ID"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerEmoji", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stickerEmoji", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .sticker(&http, StickerId::new(sid))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(s) => FnOutput::Text(s.tags.join(", ")),
        Err(e) => FnOutput::error("stickerEmoji", e),
    }
}
