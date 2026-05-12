use crate::context::{DiscordContext, FnOutput};
use chrono::Datelike;

/// Returns the current date as YYYY-MM-DD.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let now = super::helpers::now(ctx);
    FnOutput::Text(format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day()))
}
