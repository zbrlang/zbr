use crate::context::{DiscordContext, FnOutput};
use super::helpers::{member_channel_permissions, parse_permission};
use serenity::model::id::{GuildId, UserId};

/// ZhasPerm{userID;channelID;permissionName}
/// Returns "true" if the user has the provided permission in the channel, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("hasPerm", crate::error_messages::too_few_args(3, args.len()));
    }

    let user_id: u64 = match args[0].parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasPerm", crate::error_messages::expected_snowflake(1, "userID", &args[0])),
    };

    let channel_id: u64 = match args[1].parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasPerm", crate::error_messages::expected_snowflake(2, "channelID", &args[1])),
    };

    let required = match parse_permission(&args[2]) {
        Some(p) => p,
        None => return FnOutput::error("hasPerm", crate::error_messages::unknown_permission(&args[2])),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasPerm", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("hasPerm", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch member", &format!("{}", e)))?;
            
            let perms = member_channel_permissions(&http, guild_id, &member, channel_id).await?;
            Ok::<bool, String>(perms.contains(required))
        })
    });

    match result {
        Ok(has) => FnOutput::Text(has.to_string()),
        Err(e) => FnOutput::error("hasPerm", e),
    }
}
