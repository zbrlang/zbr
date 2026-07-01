use super::helpers::{
    parse_index, validate_channel_same_guild, validate_embed_sendable,
    validate_snowflake,
};
use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, EditMessage};
use serenity::model::id::{ChannelId, MessageId};

/// ZeditEmbed{channelID;messageID;index?}
/// channelID defaults to the current channel when omitted or empty.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editEmbed", crate::error_messages::action_failed_reason("edit message", "no HTTP client available")),
    };

    let (channel_id_str, message_id_str, index) = if args.len() < 2 {
        let message_id_str = args.first().filter(|s| !s.is_empty()).cloned().unwrap_or_default();
        (ctx.channel_id.clone(), message_id_str, 0usize)
    } else {
        let channel_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.channel_id.clone());
        let message_id_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
        let index = match parse_index(args.get(2), "editEmbed") {
            Ok(i) => i,
            Err(e) => return e,
        };
        (channel_id_str, message_id_str, index)
    };

    let channel_id = match validate_snowflake(&channel_id_str, "editEmbed", "channel ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    let message_id = match validate_snowflake(&message_id_str, "editEmbed", "message ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    if let Err(e) = validate_channel_same_guild(channel_id, ctx, http.clone(), "editEmbed") {
        return e;
    }

    let embed_data = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { ctx.embed.lock().await.get(index).cloned() })
    });

    let embed_data = match embed_data {
        Some(e) if e.has_content() => e,
        Some(_) => {
            return FnOutput::error("editEmbed", crate::error_messages::action_failed_reason("edit embed", &format!("embed {} has no content", index + 1)))
        }
        None => return FnOutput::error("editEmbed", crate::error_messages::not_found("embed", &(index + 1).to_string())),
    };

    if let Err(e) = validate_embed_sendable(&embed_data, "editEmbed", index) {
        return e;
    }

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut embed = CreateEmbed::new();

            if let Some(t) = &embed_data.title {
                embed = embed.title(t);
            }
            if let Some(u) = &embed_data.title_url {
                embed = embed.url(u);
            }
            if let Some(d) = &embed_data.description {
                embed = embed.description(d);
            }
            if let Some(c) = embed_data.color {
                embed = embed.color(c);
            }
            if let Some(t) = &embed_data.thumbnail {
                embed = embed.thumbnail(t);
            }
            if let Some(i) = &embed_data.image {
                embed = embed.image(i);
            }

            if let Some(f) = &embed_data.footer {
                let mut footer = CreateEmbedFooter::new(f);
                if let Some(fi) = &embed_data.footer_icon {
                    footer = footer.icon_url(fi);
                }
                embed = embed.footer(footer);
            }

            if let Some(a) = &embed_data.author {
                let mut author = CreateEmbedAuthor::new(a);
                if let Some(ai) = &embed_data.author_icon {
                    author = author.icon_url(ai);
                }
                if let Some(au) = &embed_data.author_url {
                    author = author.url(au);
                }
                embed = embed.author(author);
            }

            if embed_data.timestamp {
                embed = embed.timestamp(serenity::model::Timestamp::now());
            }

            for field in &embed_data.fields {
                embed = embed.field(&field.name, &field.value, field.inline);
            }

            let msg = EditMessage::new().embed(embed);
            match ChannelId::new(channel_id).edit_message(&http, MessageId::new(message_id), msg).await {
                Ok(_) => Ok(String::new()),
                Err(e) => Err(format!("editEmbed error: {}", e)),
            }
        })
    });

    match result {
        Err(e) => FnOutput::Error(e),
        Ok(_) => {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.consumed_embeds.lock().await.insert(index);
                })
            });
            FnOutput::Empty
        }
    }
}
