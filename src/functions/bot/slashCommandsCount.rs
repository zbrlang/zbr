use crate::context::{DiscordContext, FnOutput};
use crate::types::CommandType;

pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let cmds = crate::loader::load_commands("commands");
    let count = cmds.values().filter(|c| matches!(c.command_type, CommandType::Slash)).count();
    
    FnOutput::Text(count.to_string())
}
