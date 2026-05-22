use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::ChannelId;

/// ZthreadLocked{threadID?}
/// Returns true if the thread is locked.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid = match validate_snowflake(&cid_str, "threadLocked", "thread ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadLocked", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.guild() {
                if let Some(thread_meta) = &guild_channel.thread_metadata {
                    FnOutput::Text(thread_meta.locked.to_string())
                } else {
                    FnOutput::Text("false".to_string())
                }
            } else {
                FnOutput::Text("false".to_string())
            }
        }
        Err(_) => FnOutput::error("threadLocked", "thread not found"),
    }
}
