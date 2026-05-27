use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZeventCount{} — number of scheduled events in this server
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventCount", crate::error_messages::not_in_guild()),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("eventCount", "no HTTP client available"),
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
        Ok(events) => FnOutput::Text(events.len().to_string()),
        Err(e) => FnOutput::error("eventCount", e),
    }
}
