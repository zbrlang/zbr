use crate::context::{DiscordContext, FnOutput};
use super::helpers::member_channel_permissions;
use serenity::model::id::{GuildId, UserId};

/// ZeffectivePerms{userID;channelID}
/// Computes the final permission bitfield (including channel overwrites).
/// Returns as an integer.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("effectivePerms", crate::error_messages::too_few_args(2, args.len()));
    }

    let user_id: u64 = match args[0].parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("effectivePerms", crate::error_messages::expected_snowflake(1, "userID", &args[0])),
    };

    let channel_id: u64 = match args[1].parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("effectivePerms", crate::error_messages::expected_snowflake(2, "channelID", &args[1])),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("effectivePerms", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("effectivePerms", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch member", &format!("{}", e)))?;
            
            let perms = member_channel_permissions(&http, guild_id, &member, channel_id).await?;
            Ok::<u64, String>(perms.bits())
        })
    });

    match result {
        Ok(bits) => FnOutput::Text(bits.to_string()),
        Err(e) => FnOutput::error("effectivePerms", e),
    }
}
