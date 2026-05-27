use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId};

/// ZforumPostCount{channelID} — count of active posts in a forum channel
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumPostCount", crate::error_messages::required(1, "channelID")),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumPostCount", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumPostCount", crate::error_messages::not_in_guild()),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumPostCount", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_guild_active_threads(GuildId::new(gid))
                .await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch threads", &e.to_string()))
        })
    });
    match result {
        Ok(threads_data) => {
            let forum_channel_id = ChannelId::new(cid);
            let count = threads_data
                .threads
                .iter()
                .filter(|t| t.parent_id == Some(forum_channel_id))
                .count();
            FnOutput::Text(count.to_string())
        }
        Err(e) => FnOutput::error("forumPostCount", e),
    }
}
