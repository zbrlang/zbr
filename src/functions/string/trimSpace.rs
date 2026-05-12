use crate::context::{DiscordContext, FnOutput};

/// Removes leading and trailing whitespace.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(args[0].trim().to_string())
}
