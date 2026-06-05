use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_url;
use serenity::model::id::GuildId;

/// ZaddEmoji{name;imageURL;returnEmoji?}
/// Downloads the image, base64-encodes it, and creates a guild emoji.
/// returnEmoji: "true" returns the usable emoji string e.g. <:name:id>
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("addEmoji", crate::error_messages::required(1, "name"));
    }

    let url = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if let Err(e) = validate_url(&url, "addEmoji") { return e; }

    let return_emoji = args.get(2).filter(|s| !s.is_empty()).map(|s| s == "true").unwrap_or(false);

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("addEmoji", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("addEmoji", crate::error_messages::not_in_guild()),
    };

    let result: Result<String, String> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            // Download the image
            let bytes = reqwest::get(&url).await
                .map_err(|e| format!("failed to download image: {}", e))?
                .bytes().await
                .map_err(|e| format!("failed to read image: {}", e))?;

            // Detect content type from first bytes
            let mime = if bytes.starts_with(b"\x89PNG") { "image/png" }
                else if bytes.starts_with(b"\xff\xd8") { "image/jpeg" }
                else if bytes.starts_with(b"GIF") { "image/gif" }
                else if bytes.starts_with(b"RIFF") { "image/webp" }
                else { "image/png" };

            let b64 = base64_encode(&bytes);
            let image_data = format!("data:{};base64,{}", mime, b64);

            let emoji = GuildId::new(guild_id)
                .create_emoji(&http, &name, &image_data)
                .await
                .map_err(|e| format!("failed to create emoji: {}", e))?;

            if return_emoji {
                let prefix = if emoji.animated { "a" } else { "" };
                Ok(format!("<{}:{}:{}>", prefix, emoji.name, emoji.id))
            } else {
                Ok(String::new())
            }
        })
    });

    match result {
        Ok(s) if s.is_empty() => FnOutput::Empty,
        Ok(s) => FnOutput::Text(s),
        Err(e) => FnOutput::error("addEmoji", e),
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        out.push(CHARS[b0 >> 2] as char);
        out.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        out.push(if chunk.len() > 1 { CHARS[((b1 & 0xf) << 2) | (b2 >> 6)] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[b2 & 0x3f] as char } else { '=' });
    }
    out
}
