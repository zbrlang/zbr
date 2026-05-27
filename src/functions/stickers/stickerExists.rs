use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, StickerId};

/// ZstickerExists{stickerID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("stickerExists", crate::error_messages::required(1, "stickerID")),
    };
    let sid: u64 = match sid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerExists", crate::error_messages::expected_snowflake(1, "stickerID", &sid_str)),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerExists", crate::error_messages::not_in_guild()),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stickerExists", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async move { GuildId::new(gid).sticker(&http, StickerId::new(sid)).await })
    });
    FnOutput::Text(result.is_ok().to_string())
}
