use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, ForumTagId};

/// ZforumTagModerated{channelID;tagID} — true/false, is this tag mod-only
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagModerated", "channelID is required"),
    };
    let tid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagModerated", "tagID is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagModerated", "invalid channel ID"),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagModerated", "invalid tag ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumTagModerated", "no HTTP client available"),
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
                None => FnOutput::error("forumTagModerated", "tag not found"),
            }
        }
        Ok(_) => FnOutput::error("forumTagModerated", "not a forum channel"),
        Err(e) => FnOutput::error("forumTagModerated", e),
    }
}
