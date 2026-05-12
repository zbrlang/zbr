use crate::context::{DiscordContext, FnOutput};
use chrono::Timelike;

/// Returns the current hour (0–23).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(super::helpers::now(ctx).hour().to_string())
}
