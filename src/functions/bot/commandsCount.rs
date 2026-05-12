use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let cmds = crate::loader::load_commands("commands");
    FnOutput::Text(cmds.len().to_string())
}
