use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    // Shard latency is not exposed directly in DiscordContext's standard setup currently
    // Defaulting to 0 since serenity HTTP alone doesn't have gateway latency
    FnOutput::Text("0".to_string())
}
