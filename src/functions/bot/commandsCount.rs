use crate::context::{DiscordContext, FnOutput};
use std::collections::HashMap;

pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let registry = HashMap::new();
    let cmds = crate::loader::load_commands("commands", &registry);
    FnOutput::Text(cmds.len().to_string())
}
