use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZautomodRules{guildID?}
/// Returns a JSON array of all auto-moderation rules with their IDs and names.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if guild_id_str.is_empty() {
        return FnOutput::error("automodRules", crate::error_messages::required(1, "guild ID"));
    }

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("automodRules", crate::error_messages::expected_snowflake(1, "guild ID", &guild_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("automodRules", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id)
                .automod_rules(&http)
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(rules) => {
            let simplified: Vec<serde_json::Value> = rules
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id.to_string(),
                        "name": r.name,
                        "enabled": r.enabled,
                        "creator_id": r.creator_id.to_string(),
                    })
                })
                .collect();
            FnOutput::Text(serde_json::to_string(&simplified).unwrap_or_default())
        }
        Err(e) => FnOutput::error("automodRules", e),
    }
}
