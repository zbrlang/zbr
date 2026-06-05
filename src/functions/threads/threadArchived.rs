use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::ChannelId;

/// ZthreadArchived{threadID?}
/// Returns true if the thread is archived.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid = match validate_snowflake(&cid_str, "threadArchived", "thread ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadArchived", "no HTTP client available"),
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
                    FnOutput::Text(thread_meta.archived.to_string())
                } else {
                    FnOutput::Text("false".to_string())
                }
            } else {
                FnOutput::Text("false".to_string())
            }
        }
        Err(_) => FnOutput::error("threadArchived", crate::error_messages::not_found("thread", &cid_str)),
    }
}
