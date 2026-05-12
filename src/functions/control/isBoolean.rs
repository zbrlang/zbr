use crate::context::{DiscordContext, FnOutput};

/// ZisBoolean{value} — returns "true" if value is exactly "true" or "false".
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let val = args.get(0).map(|s| s.as_str()).unwrap_or("");
    FnOutput::Text((val == "true" || val == "false").to_string())
}
