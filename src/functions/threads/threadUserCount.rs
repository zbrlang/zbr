use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::ChannelId;

/// ZthreadUserCount{threadID}
/// Returns the number of members in the thread.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let thread_id = match validate_snowflake(args.get(0).unwrap_or(&String::new()), "threadUserCount", "thread ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadUserCount", "no HTTP client available"),
    };

    let result: Result<String, String> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let members = ChannelId::new(thread_id).get_thread_members(&http).await
                .map_err(|e| format!("failed to fetch thread members: {}", e))?;
            Ok(members.len().to_string())
        })
    });

    match result {
        Ok(count) => FnOutput::Text(count),
        Err(e) => FnOutput::error("threadUserCount", e),
    }
}
