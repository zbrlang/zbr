use crate::context::{DiscordContext, FnOutput};
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(args[0].to_uppercase())
}