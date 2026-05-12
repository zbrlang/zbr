use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZblackListRoles{roleName1;roleName2;...;(errorMessage)}
/// Halts if the author has any role matching the provided names.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("blackListRoles", "at least one role name is required");
    }

    let (role_names, error_msg) = split_last_as_error(&args);
    if role_names.is_empty() {
        return FnOutput::error("blackListRoles", "at least one role name is required");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("blackListRoles", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("blackListRoles", "not in a guild"),
    };

    let user_id: u64 = match ctx.author_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("blackListRoles", "invalid author ID"),
    };

    let names_lower: Vec<String> = role_names.iter().map(|s| s.to_lowercase()).collect();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| format!("failed to fetch member: {}", e))?;
            let roles = GuildId::new(guild_id).roles(&http).await
                .map_err(|e| format!("failed to fetch roles: {}", e))?;
            let has_role = member.roles.iter().any(|role_id| {
                roles.get(role_id)
                    .map(|r| names_lower.contains(&r.name.to_lowercase()))
                    .unwrap_or(false)
            });
            Ok::<bool, String>(has_role)
        })
    });

    match result {
        Ok(true) => FnOutput::user_error(error_msg),
        Ok(false) => FnOutput::Empty,
        Err(e) => FnOutput::error("blackListRoles", e),
    }
}

fn split_last_as_error(args: &[String]) -> (&[String], String) {
    if args.len() > 1 {
        if let Some(last) = args.last() {
            if last.contains(' ') || last.contains('!') || last.contains('.') {
                return (&args[..args.len() - 1], last.clone());
            }
        }
    }
    (args, "You are blacklisted from using this command.".to_string())
}
