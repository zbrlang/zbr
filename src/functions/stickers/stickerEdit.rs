use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditSticker;
use serenity::model::id::{GuildId, StickerId};

/// ZstickerEdit{stickerID;name?;tags?;description?}
/// Edits a sticker's metadata. Use !unchanged for fields to skip.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let sid: u64 = match sid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerEdit", crate::error_messages::expected_snowflake(1, "stickerID", &sid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerEdit", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stickerEdit", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditSticker::new();

            let name = args.get(1).map(|s| s.as_str()).unwrap_or("!unchanged");
            if name != "!unchanged" && !name.is_empty() {
                builder = builder.name(name);
            }

            let tags = args.get(2).map(|s| s.as_str()).unwrap_or("!unchanged");
            if tags != "!unchanged" && !tags.is_empty() {
                builder = builder.tags(tags);
            }

            let description = args.get(3).map(|s| s.as_str()).unwrap_or("!unchanged");
            if description != "!unchanged" {
                builder = builder.description(description);
            }

            GuildId::new(gid)
                .edit_sticker(&http, StickerId::new(sid), builder)
                .await
                .map(|_| String::new())
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("stickerEdit", e),
    }
}
