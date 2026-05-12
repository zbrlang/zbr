use crate::context::{DiscordContext, FnOutput};
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    match args.first() {
        Some(key) => FnOutput::Text(ctx.options.get(key).cloned().unwrap_or_else(|| format!("option '{}' not found", key))),
        None => FnOutput::Text("no option name provided".to_string()),
    }
}