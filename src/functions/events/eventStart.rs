use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, ScheduledEventId};

/// ZeventStart{eventID} — returns Unix timestamp
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let eid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("eventStart", crate::error_messages::required(1, "eventID")),
    };
    let eid: u64 = match eid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventStart", crate::error_messages::expected_snowflake(1, "eventID", &eid_str)),
    };
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("eventStart", crate::error_messages::not_in_guild()),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("eventStart", "no HTTP client available"),
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
        Ok(e) => FnOutput::Text(e.start_time.unix_timestamp().to_string()),
        Err(e) => FnOutput::error("eventStart", e),
    }
}
