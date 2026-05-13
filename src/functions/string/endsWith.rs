use crate::context::{DiscordContext, FnOutput};

/// ZendsWith{text;suffix}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let suffix = args.get(1).map(|s| s.as_str()).unwrap_or("");
    FnOutput::Text(text.ends_with(suffix).to_string())
}
