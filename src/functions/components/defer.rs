use crate::context::{DiscordContext, FnOutput};

/// Zdefer{} — defers the interaction response (shows "thinking...").
/// The bot then has 15 minutes to follow up.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.components.lock().await.deferred = true;
        })
    });
    FnOutput::Empty
}
