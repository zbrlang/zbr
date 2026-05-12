use crate::context::{DiscordContext, FnOutput};
use chrono::Timelike;

/// Returns the current minute (0–59).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(super::helpers::now(ctx).minute().to_string())
}
