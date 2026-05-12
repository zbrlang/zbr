use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if !ctx.command_name.is_empty() {
        FnOutput::Text(ctx.command_name.clone())
    } else if let Some(trigger) = &ctx.trigger {
        FnOutput::Text(trigger.clone())
    } else {
        FnOutput::Text(String::new())
    }
}
