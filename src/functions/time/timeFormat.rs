use crate::context::{DiscordContext, FnOutput};
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

/// ZtimeFormat{timestamp;format;timezone?}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let ts: i64 = match args.get(0).and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return FnOutput::error("timeFormat", crate::error_messages::required(1, "timestamp")),
    };
    let format = match args.get(1) {
        Some(s) => s,
        None => return FnOutput::error("timeFormat", crate::error_messages::required(2, "format")),
    };
    let timezone_str = args.get(2).map(|s| s.as_str()).unwrap_or("UTC");
    
    let tz: Tz = timezone_str.parse().unwrap_or(Tz::UTC);
    let dt = Utc.timestamp_opt(ts, 0).unwrap().with_timezone(&tz);
    
    FnOutput::Text(dt.format(format).to_string())
}
