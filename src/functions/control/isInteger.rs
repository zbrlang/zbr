use crate::context::{DiscordContext, FnOutput};

/// ZisInteger{value} — returns "true" if value parses as a whole number (no decimal point).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let val = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let result = val.parse::<i64>().is_ok();
    FnOutput::Text(result.to_string())
}
