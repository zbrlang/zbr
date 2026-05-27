use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZpostTags{threadID} — space-separated list of applied tag IDs on this forum post
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let tid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("postTags", crate::error_messages::required(1, "threadID")),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("postTags", crate::error_messages::expected_snowflake(1, "threadID", &tid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("postTags", crate::error_messages::action_failed("get HTTP client")),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_channel(ChannelId::new(tid))
                .await
                .map_err(|e| format!("failed to fetch thread: {}", e))
        })
    });
    match result {
        Ok(channel) => {
            if let serenity::model::channel::Channel::Guild(gc) = channel {
                let tags: Vec<String> = gc.applied_tags.iter().map(|t| t.to_string()).collect();
                FnOutput::Text(tags.join(" "))
            } else {
                FnOutput::Text(String::new())
            }
        }
        Err(e) => FnOutput::error("postTags", e),
    }
}
