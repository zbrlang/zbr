use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::EmojiId;

/// ZappEmojiDelete{emojiID}
/// Deletes an application emoji by ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let eid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let eid: u64 = match eid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("appEmojiDelete", crate::error_messages::expected_snowflake(1, "emoji ID", &eid_str)),
    };

    if ctx.bot_id.is_empty() {
        return FnOutput::error("appEmojiDelete", "no bot ID available");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("appEmojiDelete", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.delete_application_emoji(EmojiId::new(eid))
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("appEmojiDelete", e),
    }
}
