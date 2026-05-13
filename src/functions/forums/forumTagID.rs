use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZforumTagID{channelID;tagName} — find a tag's ID by name (case-insensitive)
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagID", "channelID is required"),
    };
    let tag_name = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagID", "tagName is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagID", "invalid channel ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumTagID", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_channel(ChannelId::new(cid))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(serenity::model::channel::Channel::Guild(gc)) => {
            let lower = tag_name.to_lowercase();
            match gc
                .available_tags
                .iter()
                .find(|t| t.name.to_lowercase() == lower)
            {
                Some(t) => FnOutput::Text(t.id.to_string()),
                None => FnOutput::Text(String::new()),
            }
        }
        Ok(_) => FnOutput::error("forumTagID", "not a forum channel"),
        Err(e) => FnOutput::error("forumTagID", e),
    }
}
