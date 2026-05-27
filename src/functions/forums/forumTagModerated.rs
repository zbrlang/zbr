use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, ForumTagId};

/// ZforumTagModerated{channelID;tagID} — true/false, is this tag mod-only
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagModerated", crate::error_messages::required(1, "channelID")),
    };
    let tid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagModerated", crate::error_messages::required(2, "tagID")),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagModerated", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagModerated", crate::error_messages::expected_snowflake(2, "tagID", &tid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumTagModerated", crate::error_messages::action_failed("get HTTP client")),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_channel(ChannelId::new(cid))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(serenity::model::channel::Channel::Guild(gc)) => {
            match gc
                .available_tags
                .iter()
                .find(|t| t.id == ForumTagId::new(tid))
            {
                Some(tag) => FnOutput::Text(tag.moderated.to_string()),
                None => FnOutput::error("forumTagModerated", crate::error_messages::not_found("tag", &tid_str)),
            }
        }
        Ok(_) => FnOutput::error("forumTagModerated", crate::error_messages::action_failed("get forum channel")),
        Err(e) => FnOutput::error("forumTagModerated", e),
    }
}
