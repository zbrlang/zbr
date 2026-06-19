use crate::context::{DiscordContext, FnOutput};

/// Ztype{value} — returns the type string.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    // This is tricky, everything is a String in args.
    // I will return "string" since that's what we have.
    FnOutput::Text("string".to_string())
}
