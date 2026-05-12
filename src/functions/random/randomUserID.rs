use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use rand::seq::SliceRandom;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("randomUserID", "no HTTP client available"),
    };
    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("randomUserID", "not in a guild"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let members = GuildId::new(guild_id).members(&http, None, None).await
                .map_err(|e| format!("failed to fetch members: {}", e))?;
            members.choose(&mut rand::thread_rng())
                .map(|m| m.user.id.to_string())
                .ok_or_else(|| "no members found".to_string())
        })
    });
    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("randomUserID", e),
    }
}
