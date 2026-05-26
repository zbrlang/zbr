use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditThread;
use serenity::model::id::ChannelId;

/// ZthreadUnarchive{threadID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let tid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("threadUnarchive", "threadID is required"),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("threadUnarchive", "invalid thread ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadUnarchive", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(tid)
                .edit_thread(&http, EditThread::new().archived(false))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("threadUnarchive", e),
    }
}
