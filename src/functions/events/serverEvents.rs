use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZserverEvents{} — space-separated list of scheduled event IDs
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("serverEvents", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverEvents", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .scheduled_events(&http, false)
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(events) => FnOutput::Text(
            events
                .iter()
                .map(|e| e.id.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        ),
        Err(e) => FnOutput::error("serverEvents", e),
    }
}
