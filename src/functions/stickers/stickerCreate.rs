use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateAttachment, CreateSticker};
use serenity::model::id::GuildId;

/// ZstickerCreate{name;fileUrl;tags;description?}
/// Creates a new sticker from a file URL. tags are comma-separated keywords (required by Discord).
/// Returns the sticker ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let file_url = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let tags = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let description = args.get(3).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    if name.is_empty() {
        return FnOutput::error("stickerCreate", crate::error_messages::required(1, "name"));
    }
    if file_url.is_empty() {
        return FnOutput::error("stickerCreate", crate::error_messages::required(2, "fileUrl"));
    }
    if tags.is_empty() {
        return FnOutput::error("stickerCreate", "tags are required (comma-separated)");
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerCreate", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stickerCreate", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let attachment = CreateAttachment::url(&http, &file_url)
                .await
                .map_err(|e| format!("failed to download file: {}", e))?;

            let builder = CreateSticker::new(&name, attachment)
                .tags(&tags)
                .description(&description);

            GuildId::new(gid)
                .create_sticker(&http, builder)
                .await
                .map(|s| s.id.to_string())
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("stickerCreate", e),
    }
}
