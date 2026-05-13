use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditScheduledEvent;
use serenity::model::id::{GuildId, ScheduledEventId};
use serenity::model::Timestamp;

/// ZeditEvent{eventID;name?;description?;startTime?}
/// Use "!unchanged" to skip a field.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let event_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editEvent", "eventID is required"),
    };
    let eid: u64 = match event_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error("editEvent", format!("invalid event ID: '{}'", event_id_str))
        }
    };

    let name = args
        .get(1)
        .filter(|s| !s.is_empty() && s.as_str() != "!unchanged")
        .cloned();
    let description = args
        .get(2)
        .filter(|s| !s.is_empty() && s.as_str() != "!unchanged")
        .cloned();
    let start_time: Option<Timestamp> = match args.get(3).map(|s| s.as_str()) {
        Some(s) if !s.is_empty() && s != "!unchanged" => {
            match s
                .parse::<i64>()
                .ok()
                .and_then(|n| Timestamp::from_unix_timestamp(n).ok())
            {
                Some(ts) => Some(ts),
                None => return FnOutput::error("editEvent", "invalid startTime timestamp"),
            }
        }
        _ => None,
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editEvent", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editEvent", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditScheduledEvent::new();
            if let Some(n) = name {
                builder = builder.name(n);
            }
            if let Some(d) = description {
                builder = builder.description(d);
            }
            if let Some(ts) = start_time {
                builder = builder.start_time(ts);
            }
            GuildId::new(gid)
                .edit_scheduled_event(&http, ScheduledEventId::new(eid), builder)
                .await
                .map_err(|e| format!("failed to edit event: {}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editEvent", e),
    }
}
