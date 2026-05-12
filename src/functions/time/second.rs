use crate::context::{DiscordContext, FnOutput};
use chrono::Timelike;

/// Returns the current second (0–59).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(super::helpers::now(ctx).second().to_string())
}
