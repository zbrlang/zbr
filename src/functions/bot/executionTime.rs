use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let elapsed = ctx.execution_start.elapsed();
    FnOutput::Text(elapsed.as_millis().to_string())
}
