use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditThread;
use serenity::model::channel::Channel;
use serenity::model::channel::ChannelFlags;
use serenity::model::id::ChannelId;

/// ZforumPostPin{postID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let pid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumPostPin", crate::error_messages::required(1, "postID")),
    };
    let pid: u64 = match pid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumPostPin", crate::error_messages::expected_snowflake(1, "postID", &pid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumPostPin", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(pid)
                .to_channel(&http)
                .await
                .map_err(|e| format!("{}", e))?;
            let current_flags = match &channel {
                Channel::Guild(gc) => gc.flags,
                _ => ChannelFlags::empty(),
            };
            let new_flags = current_flags | ChannelFlags::PINNED;
            ChannelId::new(pid)
                .edit_thread(&http, EditThread::new().flags(new_flags))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("forumPostPin", e),
    }
}
