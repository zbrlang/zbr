use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::GuildId;

/// ZserverChannels{separator?;guildID?}
/// Returns a separator-joined list of channel IDs for the server.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let sep = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "\n".to_string());
    let gid_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    let gid = match validate_snowflake(&gid_str, "serverChannels", "guild ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverChannels", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).channels(&http).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch channels", &e.to_string()))
        })
    });

    match result {
        Ok(map) => {
            let mut ids: Vec<String> = map.values().map(|c| c.id.to_string()).collect();
            ids.sort();
            FnOutput::Text(ids.join(&sep))
        }
        Err(e) => FnOutput::error("serverChannels", e),
    }
}
