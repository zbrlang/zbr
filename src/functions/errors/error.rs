use crate::context::{DiscordContext, FnOutput};

/// Zerror{message} — halts execution with a custom user-facing error message.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let msg = args.get(0).cloned().unwrap_or_default();
    FnOutput::UserError(msg)
}
