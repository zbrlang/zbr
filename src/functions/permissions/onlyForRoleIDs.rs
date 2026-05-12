use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZonlyForRoleIDs{roleID1;roleID2;...;(errorMessage)}
/// Halts unless the author has any of the provided role IDs.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForRoleIDs", "at least one role ID is required");
    }

    let (ids, error_msg) = split_ids_and_error(&args);
    if ids.is_empty() {
        return FnOutput::error("onlyForRoleIDs", "at least one role ID is required");
    }

    let role_ids: Vec<u64> = match ids.iter().map(|s| s.parse::<u64>()).collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(_) => return FnOutput::error("onlyForRoleIDs", "one or more invalid role IDs"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyForRoleIDs", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyForRoleIDs", "not in a guild"),
    };

    let user_id: u64 = match ctx.author_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyForRoleIDs", "invalid author ID"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| format!("failed to fetch member: {}", e))?;
            let has_role = member.roles.iter().any(|r| role_ids.contains(&r.get()));
            Ok::<bool, String>(has_role)
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyForRoleIDs", e),
    }
}

fn split_ids_and_error(args: &[String]) -> (&[String], String) {
    if let Some(last) = args.last() {
        if last.parse::<u64>().is_err() && args.len() > 1 {
            return (&args[..args.len() - 1], last.clone());
        }
    }
    (args, "You don't have the required role to use this command.".to_string())
}
