use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZforumTags{channelID} — space-separated list of available tag names in a forum channel
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTags", crate::error_messages::required(1, "channelID")),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTags", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumTags", crate::error_messages::action_failed("get HTTP client")),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_channel(ChannelId::new(cid))
                .await
                .map_err(|e| format!("failed to fetch channel: {}", e))
        })
    });
    match result {
        Ok(serenity::model::channel::Channel::Guild(gc)) => {
            let names: Vec<String> = gc.available_tags.iter().map(|t| t.name.clone()).collect();
            FnOutput::Text(names.join(" "))
        }
        Ok(_) => FnOutput::error("forumTags", crate::error_messages::action_failed("get forum channel")),
        Err(e) => FnOutput::error("forumTags", e),
    }
}
