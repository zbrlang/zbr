use crate::context::{DiscordContext, FnOutput};

/// ZreverseText{text}
/// Reverses the characters in a string.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    FnOutput::Text(text.chars().rev().collect())
}
