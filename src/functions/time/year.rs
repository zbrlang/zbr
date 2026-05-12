use crate::context::{DiscordContext, FnOutput};
use chrono::Datelike;

/// Returns the current year.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(super::helpers::now(ctx).year().to_string())
}
