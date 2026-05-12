use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.bot_id.clone())
}
