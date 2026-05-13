use crate::context::{DiscordContext, FnOutput};
use serenity::builder::CreateScheduledEvent;
use serenity::model::guild::ScheduledEventType;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::Timestamp;

/// ZcreateEvent{name;channelID;startTime;description?}
/// startTime is a Unix timestamp. Returns the event ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createEvent", "name is required"),
    };
    let channel_id_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createEvent", "channelID is required"),
    };
    let start_time_str = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createEvent", "startTime (Unix timestamp) is required"),
    };
    let description = args.get(3).filter(|s| !s.is_empty()).cloned();

    let cid: u64 = match channel_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error(
                "createEvent",
                format!("invalid channel ID: '{}'", channel_id_str),
            )
        }
    };
    let start_unix: i64 = match start_time_str.parse() {
        Ok(n) => n,
        Err(_) => return FnOutput::error("createEvent", "startTime must be a Unix timestamp"),
    };
    let timestamp = match Timestamp::from_unix_timestamp(start_unix) {
        Ok(ts) => ts,
        Err(_) => return FnOutput::error("createEvent", "invalid Unix timestamp"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createEvent", "not in a guild"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createEvent", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder =
                CreateScheduledEvent::new(ScheduledEventType::Voice, &name, timestamp)
                    .channel_id(ChannelId::new(cid));
            if let Some(desc) = description {
                builder = builder.description(desc);
            }
            GuildId::new(gid)
                .create_scheduled_event(&http, builder)
                .await
                .map(|e| e.id.to_string())
                .map_err(|e| format!("failed to create event: {}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("createEvent", e),
    }
}
