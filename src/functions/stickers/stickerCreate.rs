use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateAttachment, CreateSticker};
use serenity::model::id::GuildId;

/// ZstickerCreate{name;fileUrl;tags;description?}
/// Creates a new sticker from a file URL. tags are comma-separated keywords (required by Discord).
/// Returns the sticker ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned().unwrap_or_default();
    let file_url = args.get(1).cloned().unwrap_or_default();
    let tags = args.get(2).cloned().unwrap_or_default();
    let description = args.get(3).cloned().unwrap_or_default();

    if name.is_empty() {
        return FnOutput::error("stickerCreate", "name is required");
    }
    if file_url.is_empty() {
        return FnOutput::error("stickerCreate", "file URL is required");
    }
    if tags.is_empty() {
        return FnOutput::error("stickerCreate", "tags are required (comma-separated)");
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stickerCreate", "not in a guild"),
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
