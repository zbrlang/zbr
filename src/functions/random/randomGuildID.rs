use crate::context::{DiscordContext, FnOutput};
use rand::seq::IteratorRandom;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("randomGuildID", crate::error_messages::not_available("HTTP client")),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let guilds = http.get_guilds(None, None).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch guilds", &e.to_string()))?;
            guilds.iter()
                .choose(&mut rand::thread_rng())
                .map(|g| g.id.to_string())
                .ok_or_else(|| crate::error_messages::not_found("guilds", ""))
        })
    });
    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("randomGuildID", e),
    }
}
