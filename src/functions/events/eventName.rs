use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, ScheduledEventId};

/// ZeventName{eventID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let eid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("eventName", "eventID is required"),
    };
    let eid: u64 = match eid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventName", "invalid event ID"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventName", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("eventName", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .scheduled_event(&http, ScheduledEventId::new(eid), false)
                .await
                .map_err(|e| format!("failed to fetch event: {}", e))
        })
    });
    match result {
        Ok(e) => FnOutput::Text(e.name),
        Err(e) => FnOutput::error("eventName", e),
    }
}
