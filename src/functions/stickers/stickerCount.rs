use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZstickerCount{}
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerCount", crate::error_messages::not_in_guild()),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stickerCount", "no HTTP client available"),
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
        Ok(s) => FnOutput::Text(s.len().to_string()),
        Err(e) => FnOutput::error("stickerCount", e),
    }
}
