use crate::context::{DiscordContext, FnOutput};
use serenity::model::guild::ScheduledEventStatus;
use serenity::model::id::{GuildId, ScheduledEventId};

/// ZeventStatus{eventID} — returns: scheduled, active, completed, or cancelled
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let eid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("eventStatus", "eventID is required"),
    };
    let eid: u64 = match eid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventStatus", "invalid event ID"),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventStatus", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("eventStatus", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid)
                .scheduled_event(&http, ScheduledEventId::new(eid), false)
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(e) => {
            let status = match e.status {
                ScheduledEventStatus::Scheduled => "scheduled",
                ScheduledEventStatus::Active => "active",
                ScheduledEventStatus::Completed => "completed",
                ScheduledEventStatus::Canceled => "cancelled",
                _ => "unknown",
            };
            FnOutput::Text(status.to_string())
        }
        Err(e) => FnOutput::error("eventStatus", e),
    }
}
