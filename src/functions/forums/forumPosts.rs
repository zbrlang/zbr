use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId};

/// ZforumPosts{channelID} — space-separated list of active post (thread) IDs in a forum channel
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumPosts", "channelID is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumPosts", "invalid channel ID"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumPosts", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumPosts", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_guild_active_threads(GuildId::new(gid))
                .await
                .map_err(|e| format!("failed to fetch threads: {}", e))
        })
    });
    match result {
        Ok(threads_data) => {
            let forum_channel_id = ChannelId::new(cid);
            let ids: Vec<String> = threads_data
                .threads
                .iter()
                .filter(|t| t.parent_id == Some(forum_channel_id))
                .map(|t| t.id.to_string())
                .collect();
            FnOutput::Text(ids.join(" "))
        }
        Err(e) => FnOutput::error("forumPosts", e),
    }
}
