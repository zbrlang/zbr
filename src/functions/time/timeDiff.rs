use crate::context::{DiscordContext, FnOutput};

/// ZtimeDiff{timestamp1;timestamp2;unit?}
/// Returns the absolute difference between two Unix timestamps.
/// unit: seconds (default), minutes, hours, days
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let t1: i64 = match args.get(0).and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return FnOutput::error("timeDiff", crate::error_messages::required(1, "timestamp1")),
    };
    let t2: i64 = match args.get(1).and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return FnOutput::error("timeDiff", crate::error_messages::required(2, "timestamp2")),
    };
    let diff = (t1 - t2).abs();
    let unit = args.get(2).map(|s| s.as_str()).unwrap_or("seconds");
    let result = match unit {
        "minutes" => diff / 60,
        "hours" => diff / 3600,
        "days" => diff / 86400,
        _ => diff, // seconds
    };
    FnOutput::Text(result.to_string())
}
