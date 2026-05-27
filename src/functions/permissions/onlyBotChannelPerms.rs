use crate::context::{DiscordContext, FnOutput};
use super::helpers::{member_guild_permissions, parse_permission, parse_permissions};
use serenity::model::id::GuildId;

/// ZonlyBotChannelPerms{channelID;perm1;perm2;...;(errorMessage)}
/// Halts unless the bot has all provided permissions in the specified channel.
/// Note: checks guild-level permissions only (does not evaluate channel overwrites).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("onlyBotChannelPerms", crate::error_messages::too_few_args(2, args.len()));
    }

    let channel_id_str = &args[0];
    if channel_id_str.parse::<u64>().is_err() {
        return FnOutput::error("onlyBotChannelPerms", crate::error_messages::expected_snowflake(1, "channelID", channel_id_str));
    }

    let rest = &args[1..];
    let (perm_args, error_msg) = split_perms_and_error(rest);
    let required = match parse_permissions(perm_args) {
        Ok(p) => p,
        Err(e) => return FnOutput::error("onlyBotChannelPerms", e),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyBotChannelPerms", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyBotChannelPerms", crate::error_messages::not_in_guild()),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let current_user = http.get_current_user().await
                .map_err(|e| crate::error_messages::action_failed_reason("get bot user", &format!("{}", e)))?;
            let member = GuildId::new(guild_id).member(&http, current_user.id).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch bot member", &format!("{}", e)))?;
            let perms = member_guild_permissions(&http, guild_id, &member).await?;
            Ok::<bool, String>(perms.contains(required))
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyBotChannelPerms", e),
    }
}

fn split_perms_and_error(args: &[String]) -> (&[String], String) {
    if let Some(last) = args.last() {
        if parse_permission(last).is_none() && args.len() > 1 {
            return (&args[..args.len() - 1], last.clone());
        }
    }
    (args, "The bot doesn't have the required channel permissions.".to_string())
}
