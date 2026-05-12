use crate::context::{DiscordContext, FnOutput};
use std::time::{SystemTime, UNIX_EPOCH};

/// ZgetTimestamp{unit?}
/// Returns the current Unix timestamp.
/// unit: "s" (default) = seconds, "ms" = milliseconds, "ns" = nanoseconds
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let unit = match args.get(0) {
        Some(s) if !s.is_empty() => s.as_str(),
        _ => "s",
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let result = match unit {
        "s"  => now.as_secs().to_string(),
        "ms" => now.as_millis().to_string(),
        "ns" => now.as_nanos().to_string(),
        other => return FnOutput::error("getTimestamp", format!("invalid unit: '{}' (expected s, ms, or ns)", other)),
    };

    FnOutput::Text(result)
}
