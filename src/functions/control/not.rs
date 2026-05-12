use crate::context::{DiscordContext, FnOutput};

/// Znot{value}
/// Flips "true" → "false" and "false" → "true".
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let val = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let result = !(val == "true" || val == "1");
    FnOutput::Text(result.to_string())
}
