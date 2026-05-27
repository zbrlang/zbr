use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::ChannelId;
use serenity::builder::GetMessages;

/// ZthreadMessageCount{threadID}
/// Returns the total number of messages in the thread (up to 100 via API, or approximate count from thread metadata).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let thread_id = match validate_snowflake(args.get(0).unwrap_or(&String::new()), "threadMessageCount", "thread ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadMessageCount", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            // Fetch the channel to get message_count from thread metadata
            let channel = ChannelId::new(thread_id).to_channel(&http).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch thread", &e.to_string()))?;

            if let Some(_guild_channel) = channel.guild() {
                // thread_metadata message_count is approximate (capped at 50 by Discord)
                // For a more accurate count, fetch messages
                let messages = ChannelId::new(thread_id)
                    .messages(&http, GetMessages::new().limit(100)).await
                    .map_err(|e| crate::error_messages::action_failed_reason("fetch messages", &e.to_string()))?;
                Ok(messages.len().to_string())
            } else {
                Err("channel is not a thread".to_string())
            }
        })
    });

    match result {
        Ok(count) => FnOutput::Text(count),
        Err(e) => FnOutput::error("threadMessageCount", e),
    }
}
