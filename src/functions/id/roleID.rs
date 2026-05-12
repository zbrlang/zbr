use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZroleID{}       — author's top role ID
/// ZroleID{name}   — find role by name in the guild
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let query = args.get(0).cloned().unwrap_or_default();

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("roleID", "no HTTP client available"),
    };

    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("roleID", "not in a guild"),
    };

    if query.is_empty() {
        // Return author's top role
        let author_id: u64 = match ctx.author_id.parse() {
            Ok(id) => id,
            Err(_) => return FnOutput::error("roleID", "invalid author ID"),
        };

        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let member = GuildId::new(guild_id)
                    .member(&http, author_id)
                    .await
                    .map_err(|e| format!("failed to fetch member: {}", e))?;

                member.roles.first()
                    .map(|r| r.to_string())
                    .ok_or_else(|| "you have no roles".to_string())
            })
        });

        return match result {
            Ok(id) => FnOutput::Text(id),
            Err(e) => FnOutput::error("roleID", e),
        };
    }

    // Find role by name
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let roles = GuildId::new(guild_id)
                .roles(&http)
                .await
                .map_err(|e| format!("failed to fetch roles: {}", e))?;

            let q = query.to_lowercase();
            for role in roles.values() {
                if role.name.to_lowercase() == q {
                    return Ok(role.id.to_string());
                }
            }
            Err(format!("no role found matching '{}'", query))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("roleID", e),
    }
}
