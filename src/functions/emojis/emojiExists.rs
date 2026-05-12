use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{EmojiId, GuildId};

/// ZemojiExists{id}
/// Returns "true" if the emoji ID exists in the current server, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let id_str = args.get(0).cloned().unwrap_or_default();
    let emoji_id = match validate_snowflake(&id_str, "emojiExists", "emoji ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("emojiExists", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("emojiExists", "not in a guild"),
    };

    let exists = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id).emoji(&http, EmojiId::new(emoji_id)).await.is_ok()
        })
    });

    FnOutput::Text(exists.to_string())
}
