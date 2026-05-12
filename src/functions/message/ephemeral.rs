use crate::context::{DiscordContext, FnOutput};

/// Zephemeral{} — makes the slash command response only visible to the invoker.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.ephemeral.lock().await = true;
        })
    });
    FnOutput::Empty
}
