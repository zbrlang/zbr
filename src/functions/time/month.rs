use crate::context::{DiscordContext, FnOutput};
use chrono::Datelike;

/// Returns the current month (1–12).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(super::helpers::now(ctx).month().to_string())
}
