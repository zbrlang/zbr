use crate::context::{DiscordContext, FnOutput};

/// ZcurrentShard
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.shard_id.to_string())
}
