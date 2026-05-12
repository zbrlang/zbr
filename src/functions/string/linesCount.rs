use crate::context::{DiscordContext, FnOutput};

/// Returns the number of lines in the text.
/// An empty string counts as 0 lines.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let count = if text.is_empty() { 0 } else { text.lines().count() };
    FnOutput::Text(count.to_string())
}
