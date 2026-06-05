use crate::context::{DiscordContext, FnOutput};

/// ZargCount{text} — counts space-separated words in the text.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let count = if text.trim().is_empty() {
        0
    } else {
        text.split_whitespace().count()
    };
    FnOutput::Text(count.to_string())
}
