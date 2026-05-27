use crate::context::{DiscordContext, FnOutput};
use super::helpers::member_guild_permissions;
use serenity::model::id::{GuildId, UserId};
use serenity::model::permissions::Permissions;

/// ZonlyAdmin{(errorMessage)}
/// Halts unless the author has Administrator permission.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let error_msg = args.get(0).cloned()
        .unwrap_or_else(|| "You need Administrator permission to use this command.".to_string());

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyAdmin", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyAdmin", crate::error_messages::not_in_guild()),
    };

    let user_id: u64 = match ctx.author_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyAdmin", "invalid author ID"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(guild_id).member(&http, UserId::new(user_id)).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch member", &format!("{}", e)))?;
            let perms = member_guild_permissions(&http, guild_id, &member).await?;
            Ok::<bool, String>(perms.contains(Permissions::ADMINISTRATOR))
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyAdmin", e),
    }
}
