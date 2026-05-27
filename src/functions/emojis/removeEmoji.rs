use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{EmojiId, GuildId};

/// ZremoveEmoji{id}
/// Deletes a custom emoji from the server. Requires MANAGE_GUILD_EXPRESSIONS permission.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let id_str = args.get(0).cloned().unwrap_or_default();
    let emoji_id = match validate_snowflake(&id_str, "removeEmoji", "emoji ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("removeEmoji", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeEmoji", crate::error_messages::not_in_guild()),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id).delete_emoji(&http, EmojiId::new(emoji_id)).await
                .map_err(|e| format!("failed to delete emoji: {}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("removeEmoji", e),
    }
}
