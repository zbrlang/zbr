use crate::context::{DiscordContext, FnOutput};

/// ZisValidHex{value} — returns "true" if value is a valid hex color (#RRGGBB or RRGGBB).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let val = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let hex = val.trim_start_matches('#');
    let result = hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit());
    FnOutput::Text(result.to_string())
}
