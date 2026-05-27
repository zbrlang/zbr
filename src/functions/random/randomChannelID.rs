use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::channel::ChannelType;
use rand::seq::IteratorRandom;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("randomChannelID", "no HTTP client available"),
    };
    let guild_id: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("randomChannelID", crate::error_messages::not_in_guild()),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channels = GuildId::new(guild_id).channels(&http).await
                .map_err(|e| format!("failed to fetch channels: {}", e))?;
            channels.values()
                .filter(|c| c.kind == ChannelType::Text)
                .choose(&mut rand::thread_rng())
                .map(|c| c.id.to_string())
                .ok_or_else(|| "no text channels found".to_string())
        })
    });
    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("randomChannelID", e),
    }
}
