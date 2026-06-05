use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZuserID{}       — author's ID
/// ZuserID{name}   — find user by name in the guild
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let query = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if query.is_empty() {
        return FnOutput::Text(ctx.author_id.clone());
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userID", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userID", crate::error_messages::not_in_guild()),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let members = GuildId::new(guild_id)
                .members(&http, None, None)
                .await
                .map_err(|e| format!("failed to fetch members: {}", e))?;

            let q = query.to_lowercase();
            for member in &members {
                if member.user.name.to_lowercase() == q || member.display_name().to_lowercase() == q {
                    return Ok(member.user.id.to_string());
                }
            }
            Err(format!("no user found matching '{}'", query))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("userID", e),
    }
}
