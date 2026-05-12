use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

/// ZgetEmbedData{channelID;messageID;embedIndex;type}
/// type: title, description, footer, color, image, timestamp
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("getEmbedData", "channelID is required"),
    };
    let mid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("getEmbedData", "messageID is required"),
    };
    let embed_index: usize = match args.get(2) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => n,
            _ => return FnOutput::error("getEmbedData", format!("invalid embedIndex: '{}' (must be 1 or greater)", s)),
        },
        _ => return FnOutput::error("getEmbedData", "embedIndex is required"),
    };
    let data_type = match args.get(3) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("getEmbedData", "type is required"),
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getEmbedData", format!("invalid channel ID: '{}'", cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getEmbedData", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("getEmbedData", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => {
            let embed = match msg.embeds.get(embed_index - 1) {
                Some(e) => e,
                None => return FnOutput::Text(String::new()),
            };

            let value = match data_type.as_str() {
                "title" => embed.title.clone().unwrap_or_default(),
                "description" => embed.description.clone().unwrap_or_default(),
                "footer" => embed.footer.as_ref().map(|f| f.text.clone()).unwrap_or_default(),
                "color" => embed
                    .colour
                    .map(|c| format!("#{:06X}", c.0))
                    .unwrap_or_default(),
                "image" => embed.image.as_ref().map(|i| i.url.clone()).unwrap_or_default(),
                "timestamp" => embed
                    .timestamp
                    .map(|t| t.to_string())
                    .unwrap_or_default(),
                other => {
                    return FnOutput::error(
                        "getEmbedData",
                        format!("invalid type: '{}' (expected title, description, footer, color, image, or timestamp)", other),
                    )
                }
            };

            FnOutput::Text(value)
        }
        Err(_) => FnOutput::error("getEmbedData", "message not found"),
    }
}
