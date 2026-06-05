use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::channel::ChannelType;
use serenity::model::id::ChannelId;

/// ZthreadParentID{threadID?}
/// Returns the parent channel ID of the thread, or empty string.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid = match validate_snowflake(&cid_str, "threadParentID", "thread ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadParentID", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).to_channel(&http).await
        })
    });

    match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.guild() {
                match guild_channel.kind {
                    ChannelType::PublicThread | ChannelType::PrivateThread | ChannelType::NewsThread => {
                        match guild_channel.parent_id {
                            Some(id) => FnOutput::Text(id.to_string()),
                            None => FnOutput::Text(String::new()),
                        }
                    }
                    _ => FnOutput::Text(String::new()),
                }
            } else {
                FnOutput::Text(String::new())
            }
        }
        Err(_) => FnOutput::error("threadParentID", crate::error_messages::not_found("thread", &cid_str)),
    }
}
