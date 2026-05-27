use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditThread;
use serenity::model::id::{ChannelId, ForumTagId};

/// ZsetPostTags{threadID;tagID1;tagID2...}
/// Replaces all applied tags on a forum post.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let tid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("setPostTags", crate::error_messages::required(1, "threadID")),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("setPostTags", crate::error_messages::expected_snowflake(1, "threadID", &tid_str)),
    };
    let tag_ids: Vec<u64> = args
        .iter()
        .skip(1)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("setPostTags", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let tags: Vec<ForumTagId> = tag_ids.iter().map(|&id| ForumTagId::new(id)).collect();
            let builder = EditThread::new().applied_tags(tags);
            ChannelId::new(tid)
                .edit_thread(&http, builder)
                .await
                .map_err(|e| format!("failed to set tags: {}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("setPostTags", e),
    }
}
