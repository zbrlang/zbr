use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateForumTag, EditChannel};
use serenity::model::channel::ReactionType;
use serenity::model::channel::{ForumEmoji, ForumTag};
use serenity::model::id::{ChannelId, ForumTagId};

/// ZdeleteForumTag{channelID;tagID}
/// Removes an available tag from a forum channel by ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("deleteForumTag", "channelID is required"),
    };
    let tid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("deleteForumTag", "tagID is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteForumTag", "invalid channel ID"),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteForumTag", "invalid tag ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteForumTag", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = http
                .get_channel(ChannelId::new(cid))
                .await
                .map_err(|e| format!("failed to fetch channel: {}", e))?;
            let gc = match channel {
                serenity::model::channel::Channel::Guild(gc) => gc,
                _ => return Err("not a forum channel".to_string()),
            };
            let target_id = ForumTagId::new(tid);
            // Filter out the tag to delete, convert the rest back to builders
            let tags: Vec<CreateForumTag> = gc
                .available_tags
                .iter()
                .filter(|t| t.id != target_id)
                .map(forum_tag_to_create)
                .collect();
            let builder = EditChannel::new().available_tags(tags);
            ChannelId::new(cid)
                .edit(&http, builder)
                .await
                .map(|_| ())
                .map_err(|e| format!("failed to update tags: {}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("deleteForumTag", e),
    }
}

/// Convert a [`ForumTag`] model back into a [`CreateForumTag`] builder,
/// preserving name, moderated flag, and emoji.
fn forum_tag_to_create(tag: &ForumTag) -> CreateForumTag {
    let mut ct = CreateForumTag::new(&tag.name).moderated(tag.moderated);
    if let Some(ref emoji) = tag.emoji {
        match emoji {
            ForumEmoji::Name(s) => {
                ct = ct.emoji(ReactionType::Unicode(s.clone()));
            }
            ForumEmoji::Id(id) => {
                ct = ct.emoji(ReactionType::Custom {
                    id: *id,
                    animated: false,
                    name: None,
                });
            }
            _ => {}
        }
    }
    ct
}
