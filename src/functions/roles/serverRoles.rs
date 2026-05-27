use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::GuildId;

/// ZserverRoles{separator?;guildID?}
/// Returns a separator-joined list of role IDs for the server.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sep = args.get(0).cloned().unwrap_or_else(|| "\n".to_string());
    let gid_str = args.get(1).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    let gid = match validate_snowflake(&gid_str, "serverRoles", "guild ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverRoles", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).roles(&http).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch roles", &e.to_string()))
        })
    });

    match result {
        Ok(map) => {
            // roles is a HashMap<RoleId, Role>; sort by position for stable order
            let mut roles: Vec<_> = map.values().cloned().collect();
            roles.sort_by_key(|r| r.position);
            let ids: Vec<String> = roles.into_iter().map(|r| r.id.to_string()).collect();
            FnOutput::Text(ids.join(&sep))
        }
        Err(e) => FnOutput::error("serverRoles", e),
    }
}
