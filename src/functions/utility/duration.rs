use crate::context::{DiscordContext, FnOutput};

/// Zduration{seconds}
/// Formats a number of seconds into a human-readable string like "2d 5h 30m 15s".
/// Only non-zero units are shown.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let secs_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if secs_str.is_empty() {
        return FnOutput::error("duration", crate::error_messages::required(1, "seconds"));
    }

    let secs: i64 = match secs_str.parse() {
        Ok(s) => s,
        Err(_) => return FnOutput::error("duration", crate::error_messages::expected_number(1, "seconds", &secs_str)),
    };

    if secs < 0 {
        return FnOutput::error("duration", crate::error_messages::must_be_positive(1, "seconds", secs));
    }

    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;

    let mut parts = Vec::new();
    if d > 0 { parts.push(format!("{}d", d)); }
    if h > 0 { parts.push(format!("{}h", h)); }
    if m > 0 { parts.push(format!("{}m", m)); }
    if s > 0 || parts.is_empty() { parts.push(format!("{}s", s)); }

    FnOutput::Text(parts.join(" "))
}
