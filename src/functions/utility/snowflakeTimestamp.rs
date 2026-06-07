use crate::context::{DiscordContext, FnOutput};

/// ZsnowflakeTimestamp{snowflakeID}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("snowflakeTimestamp", crate::error_messages::required(1, "snowflakeID")),
    };

    let id: u64 = match id_str.parse() {
        Ok(val) => val,
        Err(_) => return FnOutput::error("snowflakeTimestamp", crate::error_messages::expected_snowflake(1, "snowflakeID", id_str)),
    };

    // Discord snowflake timestamp: (id >> 22) + 1420070400000
    let timestamp_ms = (id >> 22) + 1420070400000;
    let timestamp_secs = timestamp_ms / 1000;

    FnOutput::Text(timestamp_secs.to_string())
}
