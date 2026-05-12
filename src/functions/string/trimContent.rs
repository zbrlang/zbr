use crate::context::{DiscordContext, FnOutput};

/// Removes duplicate spaces — collapses multiple consecutive spaces into one.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    // Split on whitespace and rejoin with single spaces (also trims edges)
    let result = args[0].split_whitespace().collect::<Vec<_>>().join(" ");
    FnOutput::Text(result)
}
