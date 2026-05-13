use crate::context::{DiscordContext, FnOutput};
use serenity::model::channel::ForumEmoji;
use serenity::model::id::{ChannelId, ForumTagId};

/// ZforumTagEmoji{channelID;tagID} — emoji of a tag (unicode character or custom emoji ID)
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagEmoji", "channelID is required"),
    };
    let tid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("forumTagEmoji", "tagID is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagEmoji", "invalid channel ID"),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("forumTagEmoji", "invalid tag ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("forumTagEmoji", "no HTTP client available"),
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
                Some(tag) => {
                    let emoji = match &tag.emoji {
                        Some(ForumEmoji::Name(s)) => s.clone(),
                        Some(ForumEmoji::Id(id)) => id.to_string(),
                        None => String::new(),
                        Some(_) => String::new(),
                    };
                    FnOutput::Text(emoji)
                }
                None => FnOutput::error("forumTagEmoji", "tag not found"),
            }
        }
        Ok(_) => FnOutput::error("forumTagEmoji", "not a forum channel"),
        Err(e) => FnOutput::error("forumTagEmoji", e),
    }
}
