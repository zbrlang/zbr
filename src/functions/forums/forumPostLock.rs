use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditThread;
use serenity::model::id::ChannelId;

/// ZforumPostLock{postID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let pid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumPostLock", crate::error_messages::required(1, "postID")),
    };
    let pid: u64 = match pid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumPostLock", crate::error_messages::expected_snowflake(1, "postID", &pid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumPostLock", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(pid)
                .edit_thread(&http, EditThread::new().locked(true))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("forumPostLock", e),
    }
}
