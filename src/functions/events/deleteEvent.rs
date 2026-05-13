use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, ScheduledEventId};

/// ZdeleteEvent{eventID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let eid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("deleteEvent", "eventID is required"),
    };
    let eid: u64 = match eid_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("deleteEvent", format!("invalid event ID: '{}'", eid_str))
        }
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteEvent", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteEvent", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .delete_scheduled_event(&http, ScheduledEventId::new(eid))
                .await
                .map_err(|e| format!("failed to delete event: {}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("deleteEvent", e),
    }
}
