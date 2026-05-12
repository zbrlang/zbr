use crate::context::{DiscordContext, FnOutput};
use chrono::Datelike;

/// Returns the current day of the month (1–31).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(super::helpers::now(ctx).day().to_string())
}
