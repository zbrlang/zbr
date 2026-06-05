use crate::context::{DiscordContext, FnOutput};
use std::collections::HashMap;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "\n".to_string());
    let registry = HashMap::new();
    let cmds = crate::loader::load_commands("commands", &registry);
    
    let mut triggers: Vec<String> = cmds.keys().cloned().collect();
    triggers.sort();
    
    FnOutput::Text(triggers.join(&separator))
}
