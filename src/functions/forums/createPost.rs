use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateForumPost, CreateMessage};
use serenity::model::id::{ChannelId, ForumTagId};

/// ZcreatePost{channelID;title;content?;tagIDs?}
/// tagIDs: space-separated list of forum tag IDs to apply.
/// Returns the thread (post) ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createPost", "channelID is required"),
    };
    let title = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createPost", "title is required"),
    };
    let content = args.get(2).cloned().unwrap_or_default();
    let tag_ids: Vec<u64> = args
        .get(3)
        .map(|s| {
            s.split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect()
        })
        .unwrap_or_default();

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createPost", "invalid channel ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createPost", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let message = CreateMessage::new().content(content);
            let mut builder = CreateForumPost::new(&title, message);
            for tag_id in tag_ids {
                builder = builder.add_applied_tag(ForumTagId::new(tag_id));
            }
            ChannelId::new(cid)
                .create_forum_post(&http, builder)
                .await
                .map(|thread| thread.id.to_string())
                .map_err(|e| format!("failed to create post: {}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("createPost", e),
    }
}
