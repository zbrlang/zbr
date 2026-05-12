use crate::context::{DiscordContext, FnOutput};
use super::helpers::{member_guild_permissions, parse_permission, parse_permissions};
use serenity::model::id::{GuildId, UserId};

/// ZonlyPerms{perm1;perm2;...;(errorMessage)}
/// Halts unless the author has all provided permissions.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyPerms", "at least one permission is required");
    }

    let (perm_args, error_msg) = split_perms_and_error(&args);
    let required = match parse_permissions(perm_args) {
        Ok(p) => p,
        Err(e) => return FnOutput::error("onlyPerms", e),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyPerms", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyPerms", "not in a guild"),
    };

    let user_id: u64 = match ctx.author_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyPerms", "invalid author ID"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| format!("failed to fetch member: {}", e))?;
            let perms = member_guild_permissions(&http, guild_id, &member).await?;
            Ok::<bool, String>(perms.contains(required))
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyPerms", e),
    }
}

fn split_perms_and_error(args: &[String]) -> (&[String], String) {
    if let Some(last) = args.last() {
        if parse_permission(last).is_none() && args.len() > 1 {
            return (&args[..args.len() - 1], last.clone());
        }
    }
    (args, "You don't have permission to use this command.".to_string())
}
