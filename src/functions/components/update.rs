use crate::context::{DiscordContext, FnOutput};

/// Zupdate{} — signals that this interaction response should update the original message.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.components.lock().await.update_message = true;
        })
    });
    FnOutput::Empty
}
