use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let count = ctx.cache.guilds().len();
    FnOutput::Text(count.to_string())
}
