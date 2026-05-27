use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateForumTag, EditChannel};
use serenity::model::channel::ReactionType;
use serenity::model::channel::{ForumEmoji, ForumTag};
use serenity::model::id::{ChannelId, ForumTagId};

/// ZeditForumTag{channelID;tagID;name?;moderated?}
/// Edits an existing available tag in a forum channel by ID.
/// Use "!unchanged" for any field you want to leave as-is.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editForumTag", crate::error_messages::required(1, "channelID")),
    };
    let tid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editForumTag", crate::error_messages::required(2, "tagID")),
    };
    let new_name = args
        .get(2)
        .filter(|s| !s.is_empty() && s.as_str() != "!unchanged")
        .cloned();
    let new_moderated: Option<bool> = match args.get(3).map(|s| s.as_str()) {
        Some(s) if s != "!unchanged" && !s.is_empty() => Some(s == "true"),
        _ => None,
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editForumTag", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let tid: u64 = match tid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editForumTag", crate::error_messages::expected_snowflake(2, "tagID", &tid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editForumTag", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = http
                .get_channel(ChannelId::new(cid))
                .await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch channel", &e.to_string()))?;
            let gc = match channel {
                serenity::model::channel::Channel::Guild(gc) => gc,
                _ => return Err("not a forum channel".to_string()),
            };
            let target_id = ForumTagId::new(tid);
            if !gc.available_tags.iter().any(|t| t.id == target_id) {
                return Err(crate::error_messages::not_found("tag", &tid_str));
            }
            let tags: Vec<CreateForumTag> = gc
                .available_tags
                .iter()
                .map(|t| {
                    if t.id == target_id {
                        // Apply edits to this tag
                        let name = new_name.as_deref().unwrap_or(&t.name);
                        let moderated = new_moderated.unwrap_or(t.moderated);
                        let mut ct = CreateForumTag::new(name).moderated(moderated);
                        // Preserve existing emoji
                        if let Some(ref emoji) = t.emoji {
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
                    } else {
                        forum_tag_to_create(t)
                    }
                })
                .collect();
            let builder = EditChannel::new().available_tags(tags);
            ChannelId::new(cid)
                .edit(&http, builder)
                .await
                .map(|_| ())
                .map_err(|e| crate::error_messages::action_failed_reason("update tags", &e.to_string()))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editForumTag", e),
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
