use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage};
use serenity::model::id::ChannelId;
use super::helpers::{parse_index, validate_bool, validate_embed_sendable, validate_snowflake};

// args: channel_id ; embed_index (default 1) ; return_id (optional "true"/"false")
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("sendEmbed", "no HTTP client available"),
    };

    let channel_id_str = args.get(0).cloned().unwrap_or_default();
    let channel_id = match validate_snowflake(&channel_id_str, "sendEmbed", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let index = match parse_index(args.get(1), "sendEmbed") {
        Ok(i) => i, Err(e) => return e,
    };

    let return_id = match args.get(2) {
        Some(s) => match validate_bool(s, "sendEmbed") {
            Ok(b) => b, Err(e) => return e,
        },
        None => false,
    };

    // Validate embed exists and has content before touching Discord
    let embed_data = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.embed.lock().await.get(index).cloned()
        })
    });

    let embed_data = match embed_data {
        Some(e) if e.has_content() => e,
        Some(_) => return FnOutput::error("sendEmbed", format!("embed {} has no content", index + 1)),
        None    => return FnOutput::error("sendEmbed", format!("embed {} does not exist", index + 1)),
    };

    // Validate Discord's minimum requirements before making any API call
    if let Err(e) = validate_embed_sendable(&embed_data, "sendEmbed", index) { return e; }

    // Build and send
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut embed = CreateEmbed::new();

            if let Some(t) = &embed_data.title { embed = embed.title(t); }
            if let Some(u) = &embed_data.title_url { embed = embed.url(u); }
            if let Some(d) = &embed_data.description { embed = embed.description(d); }
            if let Some(c) = embed_data.color { embed = embed.color(c); }
            if let Some(t) = &embed_data.thumbnail { embed = embed.thumbnail(t); }
            if let Some(i) = &embed_data.image { embed = embed.image(i); }

            if let Some(f) = &embed_data.footer {
                let mut footer = CreateEmbedFooter::new(f);
                if let Some(fi) = &embed_data.footer_icon { footer = footer.icon_url(fi); }
                embed = embed.footer(footer);
            }

            if let Some(a) = &embed_data.author {
                let mut author = CreateEmbedAuthor::new(a);
                if let Some(ai) = &embed_data.author_icon { author = author.icon_url(ai); }
                if let Some(au) = &embed_data.author_url { author = author.url(au); }
                embed = embed.author(author);
            }

            if embed_data.timestamp {
                embed = embed.timestamp(serenity::model::Timestamp::now());
            }

            for field in &embed_data.fields {
                embed = embed.field(&field.name, &field.value, field.inline);
            }

            let msg = CreateMessage::new().add_embed(embed);
            match ChannelId::new(channel_id).send_message(&http, msg).await {
                Ok(m) => Ok(if return_id { m.id.to_string() } else { String::new() }),
                Err(e) => Err(format!("sendEmbed error: {}", e)),
            }
        })
    });

    match result {
        Err(e) => FnOutput::Error(e),
        Ok(id) => {
            // Mark consumed only after a successful send
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.consumed_embeds.lock().await.insert(index);
                })
            });
            if id.is_empty() { FnOutput::Empty } else { FnOutput::Text(id) }
        }
    }
}
