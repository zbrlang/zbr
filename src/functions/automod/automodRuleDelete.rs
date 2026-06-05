use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RuleId};

/// ZautomodRuleDelete{guildID;ruleID}
/// Deletes an auto-moderation rule.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let rule_id_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("automodRuleDelete", crate::error_messages::expected_snowflake(1, "guild ID", &guild_id_str)),
    };

    let rule_id: u64 = match rule_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("automodRuleDelete", crate::error_messages::expected_snowflake(2, "rule ID", &rule_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("automodRuleDelete", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(guild_id)
                .delete_automod_rule(&http, RuleId::new(rule_id))
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("automodRuleDelete", e),
    }
}
