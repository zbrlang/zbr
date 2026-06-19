use crate::context::{DiscordContext, FnOutput};

/// ZwordCount{text}
/// Returns word count.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let count = text.split_whitespace().count();
    FnOutput::Text(count.to_string())
}
