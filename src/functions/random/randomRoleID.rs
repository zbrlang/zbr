use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use rand::seq::IteratorRandom;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("randomRoleID", "no HTTP client available"),
    };
    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("randomRoleID", "not in a guild"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let roles = GuildId::new(guild_id).roles(&http).await
                .map_err(|e| format!("failed to fetch roles: {}", e))?;
            roles.values()
                .choose(&mut rand::thread_rng())
                .map(|r| r.id.to_string())
                .ok_or_else(|| "no roles found".to_string())
        })
    });
    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("randomRoleID", e),
    }
}
