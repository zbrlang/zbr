use crate::context::{DiscordContext, FnOutput};
use super::helpers::{member_guild_permissions, parse_permissions};
use serenity::model::id::{GuildId, UserId};

/// ZcheckUserPerms{userID;perm1;perm2;...}
/// Returns "true" if the user has all provided permissions, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("checkUserPerms", "user ID is required");
    }

    let user_id: u64 = match args[0].parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("checkUserPerms", format!("invalid user ID: '{}'", args[0])),
    };

    let required = match parse_permissions(&args[1..]) {
        Ok(p) => p,
        Err(e) => return FnOutput::error("checkUserPerms", e),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("checkUserPerms", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("checkUserPerms", "not in a guild"),
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
        Ok(has) => FnOutput::Text(has.to_string()),
        Err(e) => FnOutput::error("checkUserPerms", e),
    }
}
