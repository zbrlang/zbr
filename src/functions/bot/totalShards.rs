use crate::context::{DiscordContext, FnOutput};

/// ZtotalShards
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.total_shards.to_string())
}
