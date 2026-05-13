use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZserverStickers{} — space-separated list of sticker IDs
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("serverStickers", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverStickers", "no HTTP client available"),
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
        Ok(stickers) => FnOutput::Text(
            stickers
                .iter()
                .map(|s| s.id.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        ),
        Err(e) => FnOutput::error("serverStickers", e),
    }
}
