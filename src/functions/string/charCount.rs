use crate::context::{DiscordContext, FnOutput};

/// Returns the number of Unicode characters (not bytes) in the text.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(args[0].chars().count().to_string())
}
