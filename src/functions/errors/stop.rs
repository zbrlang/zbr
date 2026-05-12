use crate::context::{DiscordContext, FnOutput};

/// Zstop{} — halts execution silently. No error, no output.
pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    // UserError with empty string halts execution without showing anything
    FnOutput::UserError(String::new())
}
