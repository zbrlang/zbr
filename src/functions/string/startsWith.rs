use crate::context::{DiscordContext, FnOutput};

/// ZstartsWith{text;prefix}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let prefix = args.get(1).map(|s| s.as_str()).unwrap_or("");
    FnOutput::Text(text.starts_with(prefix).to_string())
}
