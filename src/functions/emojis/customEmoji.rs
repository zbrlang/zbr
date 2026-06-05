use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZcustomEmoji{name}
/// Returns the usable emoji string for a guild emoji by name, e.g. <:wave:123456>
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("customEmoji", crate::error_messages::required(1, "name"));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("customEmoji", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("customEmoji", crate::error_messages::not_in_guild()),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let emojis = GuildId::new(guild_id).emojis(&http).await
                .map_err(|e| format!("failed to fetch emojis: {}", e))?;

            let name_lower = name.to_lowercase();
            emojis.into_iter()
                .find(|e| e.name.to_lowercase() == name_lower)
                .map(|e| {
                    let prefix = if e.animated { "a" } else { "" };
                    format!("<{}:{}:{}>", prefix, e.name, e.id)
                })
                .ok_or_else(|| format!("no emoji found with name '{}'", name))
        })
    });

    match result {
        Ok(s) => FnOutput::Text(s),
        Err(e) => FnOutput::error("customEmoji", e),
    }
}
