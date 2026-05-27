use crate::context::{DiscordContext, FnOutput};
use serenity::builder::CreateAttachment;

/// ZappEmojiCreate{name;imageURL}
/// Creates a new application emoji from an image URL. Returns the emoji ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned().unwrap_or_default();
    let image_url = args.get(1).cloned().unwrap_or_default();

    if name.is_empty() {
        return FnOutput::error("appEmojiCreate", crate::error_messages::required(1, "name"));
    }
    if image_url.is_empty() {
        return FnOutput::error("appEmojiCreate", crate::error_messages::required(2, "image URL"));
    }

    if ctx.bot_id.is_empty() {
        return FnOutput::error("appEmojiCreate", "no bot ID available");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("appEmojiCreate", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let attachment = CreateAttachment::url(&http, &image_url)
                .await
                .map_err(|e| format!("failed to download image: {}", e))?;

            http.create_application_emoji(
                &serde_json::json!({
                    "name": name,
                    "image": attachment.to_base64(),
                }),
            )
            .await
            .map(|e| e.id.to_string())
            .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("appEmojiCreate", e),
    }
}
