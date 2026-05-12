use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZemoteCount{}
/// Returns the total number of custom emojis (static + animated) in the server.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("emoteCount", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("emoteCount", "not in a guild"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id).emojis(&http).await
                .map(|e| e.len().to_string())
                .map_err(|e| format!("failed to fetch emojis: {}", e))
        })
    });

    match result {
        Ok(count) => FnOutput::Text(count),
        Err(e) => FnOutput::error("emoteCount", e),
    }
}
