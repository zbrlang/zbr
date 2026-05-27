use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZonlyForRoles{roleName1;roleName2;...;(errorMessage)}
/// Halts unless the author has any of the provided roles (matched by name).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForRoles", crate::error_messages::too_few_args(1, args.len()));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyForRoles", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyForRoles", crate::error_messages::not_in_guild()),
    };

    let user_id: u64 = match ctx.author_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyForRoles", "invalid author ID"),
    };

    // Last arg is error message if it doesn't look like a role name that exists
    // (we can't validate without an API call, so just check if last arg is non-empty)
    let (role_names, error_msg) = split_last_as_error(&args);

    let names_lower: Vec<String> = role_names.iter().map(|s| s.to_lowercase()).collect();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch member", &format!("{}", e)))?;
            let roles = GuildId::new(guild_id).roles(&http).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch roles", &format!("{}", e)))?;

            let has_role = member.roles.iter().any(|role_id| {
                roles.get(role_id)
                    .map(|r| names_lower.contains(&r.name.to_lowercase()))
                    .unwrap_or(false)
            });
            Ok::<bool, String>(has_role)
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyForRoles", e),
    }
}

fn split_last_as_error(args: &[String]) -> (&[String], String) {
    // Heuristic: if there are 2+ args and the last one contains spaces or
    // punctuation typical of a sentence, treat it as the error message.
    if args.len() > 1 {
        if let Some(last) = args.last() {
            if last.contains(' ') || last.contains('!') || last.contains('.') {
                return (&args[..args.len() - 1], last.clone());
            }
        }
    }
    (args, "You don't have the required role to use this command.".to_string())
}
