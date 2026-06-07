use crate::context::{DiscordContext, FnOutput};

/// ZsnowflakeAge{snowflakeID}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("snowflakeAge", crate::error_messages::required(1, "snowflakeID")),
    };

    let id: u64 = match id_str.parse() {
        Ok(val) => val,
        Err(_) => return FnOutput::error("snowflakeAge", crate::error_messages::expected_snowflake(1, "snowflakeID", id_str)),
    };

    let timestamp_ms = (id >> 22) + 1420070400000;
    let now_ms = chrono::Utc::now().timestamp_millis() as u64;

    if now_ms < timestamp_ms {
        return FnOutput::Text("0 seconds ago".to_string());
    }

    let diff_secs = (now_ms - timestamp_ms) / 1000;

    let age = if diff_secs < 60 {
        format!("{} seconds ago", diff_secs)
    } else if diff_secs < 3600 {
        format!("{} minutes ago", diff_secs / 60)
    } else if diff_secs < 86400 {
        format!("{} hours ago", diff_secs / 3600)
    } else if diff_secs < 2592000 {
        format!("{} days ago", diff_secs / 86400)
    } else if diff_secs < 31536000 {
        format!("{} months ago", diff_secs / 2592000)
    } else {
        format!("{} years ago", diff_secs / 31536000)
    };

    FnOutput::Text(age)
}
