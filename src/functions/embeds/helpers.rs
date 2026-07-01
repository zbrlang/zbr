use crate::context::{ DiscordContext, Embed, FnOutput };
use serenity::model::id::ChannelId;
use std::sync::Arc;
use url::Url;

/// Parse a 1-based embed index argument, defaulting to 1, returning 0-based.
/// Returns `FnOutput::Error` if the value is out of range (> 10).
pub fn parse_index(s: Option<&String>, fn_name: &str) -> Result<usize, FnOutput> {
    let one_based = s.and_then(|v| v.parse::<usize>().ok()).unwrap_or(1);

    if one_based == 0 || one_based > 10 {
        return Err(
            FnOutput::error(
                fn_name,
                format!("embed index must be between 1 and 10, got {}", one_based)
            )
        );
    }

    Ok(one_based.saturating_sub(1))
}

/// Validate a URL: must be a valid absolute URL with http or https scheme.
pub fn validate_url(url_str: &str, fn_name: &str) -> Result<(), FnOutput> {
    match Url::parse(url_str) {
        Ok(parsed) if parsed.scheme() == "http" || parsed.scheme() == "https" => Ok(()),
        Ok(_) =>
            Err(
                FnOutput::error(
                    fn_name,
                    format!("invalid URL scheme in '{}' (must be http or https)", url_str)
                )
            ),
        Err(_) => Err(FnOutput::error(fn_name, format!("invalid URL: '{}'", url_str))),
    }
}

/// Validate a Discord snowflake ID: must parse as a non-zero u64.
pub fn validate_snowflake(id: &str, fn_name: &str, label: &str) -> Result<u64, FnOutput> {
    match id.parse::<u64>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(FnOutput::error(fn_name, format!("invalid {}: '{}'", label, id))),
    }
}

/// Ensure the provided channel is inside the current guild when a guild context exists.
pub fn validate_channel_same_guild(
    channel_id: u64,
    ctx: &DiscordContext,
    http: Arc<serenity::http::Http>,
    fn_name: &str
) -> Result<(), FnOutput> {
    if ctx.guild_id.is_empty() {
        return Ok(());
    }

    let guild_id = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match ChannelId::new(channel_id).to_channel(&http).await {
                Ok(serenity::model::channel::Channel::Guild(channel)) => {
                    Ok(channel.guild_id.to_string())
                }
                _ => Err(()),
            }
        })
    });

    match guild_id {
        Ok(gid) if gid == ctx.guild_id => Ok(()),
        Ok(_) => Err(FnOutput::Empty), // Silent failure for cross-guild attempts
        Err(_) => Err(FnOutput::error(fn_name, "channel not found or not a guild channel")),
    }
}

/// Validate a hex color string (#rrggbb or rrggbb). Returns the u32 value.
pub fn validate_color(hex: &str, fn_name: &str) -> Result<u32, FnOutput> {
    let stripped = hex.trim_start_matches('#');
    if stripped.is_empty() || stripped.len() > 6 {
        return Err(FnOutput::error(fn_name, format!("invalid hex color: '{}'", hex)));
    }
    u32::from_str_radix(stripped, 16).map_err(|_|
        FnOutput::error(fn_name, format!("invalid hex color: '{}'", hex))
    )
}

/// Validate a boolean string: must be exactly "true" or "false".
pub fn validate_bool(s: &str, fn_name: &str) -> Result<bool, FnOutput> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ =>
            Err(
                FnOutput::error(
                    fn_name,
                    format!("invalid boolean: '{}' (expected 'true' or 'false')", s)
                )
            ),
    }
}

// ── Embed store helpers ───────────────────────────────────────────────────────

/// Ensure the embed vec is large enough for `index` (0-based), then call `f`
/// with a mutable reference to the embed at that index.
pub fn with_embed<F>(ctx: &DiscordContext, index: usize, f: F) where F: FnOnce(&mut Embed) {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut embeds = ctx.embed.lock().await;
            if embeds.len() <= index {
                embeds.resize_with(index + 1, Default::default);
            }
            f(&mut embeds[index]);
        })
    });
}

/// Read a field from the embed at `index` without mutating it.
/// Returns `None` if the index is out of bounds.
pub fn read_embed<F, T>(ctx: &DiscordContext, index: usize, f: F) -> Option<T>
    where F: FnOnce(&Embed) -> Option<T>
{
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle
            ::current()
            .block_on(async { ctx.embed.lock().await.get(index).and_then(f) })
    })
}

/// Validate that an embed meets Discord's minimum requirements before sending.
/// Discord requires at least one of: title, description, author name, or a field.
pub fn validate_embed_sendable(embed: &Embed, fn_name: &str, index: usize) -> Result<(), FnOutput> {
    let has_required =
        embed.title.is_some() ||
        embed.description.is_some() ||
        embed.author.is_some() ||
        !embed.fields.is_empty();

    if !has_required {
        return Err(
            FnOutput::error(
                fn_name,
                format!(
                    "embed {} must have at least a title, description, author, or field before it can be sent",
                    index + 1
                )
            )
        );
    }
    Ok(())
}
