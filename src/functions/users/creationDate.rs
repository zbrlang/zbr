use crate::context::{DiscordContext, FnOutput};
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

/// ZcreationDate{id;format}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    if id_str.is_empty() {
        return FnOutput::error("creationDate", crate::error_messages::required(1, "ID"));
    }
    let format_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => "%Y-%m-%d".to_string(),
    };

    let id: u64 = match id_str.parse() {
        Ok(v) => v,
        Err(_) => return FnOutput::error("creationDate", crate::error_messages::expected_snowflake(1, "ID", &id_str)),
    };

    let timestamp_ms = (id >> 22) + 1420070400000;
    
    let tz_str = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.timezone.lock().await.clone()
        })
    });

    let tz: Tz = tz_str.parse().unwrap_or(chrono_tz::Asia::Tokyo);

    if let chrono::LocalResult::Single(dt) = Utc.timestamp_opt((timestamp_ms / 1000) as i64, ((timestamp_ms % 1000) * 1000000) as u32) {
        FnOutput::Text(dt.with_timezone(&tz).format(&format_str).to_string())
    } else {
        FnOutput::error("creationDate", "invalid timestamp generated")
    }
}
