use crate::context::{DiscordContext, FnOutput};

/// ZbyteCount{text} — returns the number of UTF-8 bytes in the text.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    FnOutput::Text(text.len().to_string())
}
