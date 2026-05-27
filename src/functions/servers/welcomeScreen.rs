use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZwelcomeScreen{guildID?}
/// Returns the guild's welcome screen as JSON, or empty string if not set.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| ctx.guild_id.clone());
    if guild_id_str.is_empty() {
        return FnOutput::error("welcomeScreen", crate::error_messages::not_in_guild());
    }

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("welcomeScreen", crate::error_messages::expected_snowflake(1, "guild ID", &guild_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("welcomeScreen", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id)
                .get_welcome_screen(&http)
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(screen) => FnOutput::Text(serde_json::to_string(&screen).unwrap_or_default()),
        Err(e) => FnOutput::error("welcomeScreen", e),
    }
}
