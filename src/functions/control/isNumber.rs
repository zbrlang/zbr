use crate::context::{DiscordContext, FnOutput};

/// ZisNumber{value} — returns "true" if value parses as any number (integer or float).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let val = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let result = val.parse::<f64>().is_ok();
    FnOutput::Text(result.to_string())
}
