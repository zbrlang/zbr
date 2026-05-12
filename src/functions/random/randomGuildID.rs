use crate::context::{DiscordContext, FnOutput};
use rand::seq::IteratorRandom;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("randomGuildID", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let guilds = http.get_guilds(None, None).await
                .map_err(|e| format!("failed to fetch guilds: {}", e))?;
            guilds.iter()
                .choose(&mut rand::thread_rng())
                .map(|g| g.id.to_string())
                .ok_or_else(|| "no guilds found".to_string())
        })
    });
    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("randomGuildID", e),
    }
}
