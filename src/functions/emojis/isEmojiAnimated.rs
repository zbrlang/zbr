use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{EmojiId, GuildId};

/// ZisEmojiAnimated{id}
/// Returns "true" if the emoji is animated, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let id_str = args.get(0).cloned().unwrap_or_default();
    let emoji_id = match validate_snowflake(&id_str, "isEmojiAnimated", "emoji ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isEmojiAnimated", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isEmojiAnimated", crate::error_messages::not_in_guild()),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id).emoji(&http, EmojiId::new(emoji_id)).await
                .map(|e| e.animated.to_string())
                .map_err(|e| format!("emoji not found: {}", e))
        })
    });

    match result {
        Ok(s) => FnOutput::Text(s),
        Err(e) => FnOutput::error("isEmojiAnimated", e),
    }
}
