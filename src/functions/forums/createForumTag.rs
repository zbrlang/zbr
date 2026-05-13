use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateForumTag, EditChannel};
use serenity::model::channel::ReactionType;
use serenity::model::channel::{ForumEmoji, ForumTag};
use serenity::model::id::ChannelId;

/// ZcreateForumTag{channelID;name;emoji?;moderated?}
/// Adds a new available tag to a forum channel.
/// emoji: a unicode emoji string (e.g. "🔥"). Custom emojis are not supported via text args.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createForumTag", "channelID is required"),
    };
    let name = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createForumTag", "name is required"),
    };
    let emoji = args
        .get(2)
        .filter(|s| !s.is_empty() && s.as_str() != "!unchanged")
        .cloned();
    let moderated: bool = args.get(3).map(|s| s == "true").unwrap_or(false);
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createForumTag", "invalid channel ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createForumTag", "no HTTP client available"),
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
            // Convert existing tags back to CreateForumTag builders
            let mut tags: Vec<CreateForumTag> =
                gc.available_tags.iter().map(forum_tag_to_create).collect();
            // Build the new tag
            let mut new_tag = CreateForumTag::new(&name).moderated(moderated);
            if let Some(ref e) = emoji {
                new_tag = new_tag.emoji(ReactionType::Unicode(e.clone()));
            }
            tags.push(new_tag);
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
        Err(e) => FnOutput::error("createForumTag", e),
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
