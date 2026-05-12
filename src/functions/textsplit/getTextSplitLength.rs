use crate::context::{DiscordContext, FnOutput};

/// ZgetTextSplitLength{}
/// Returns the number of elements in the current split.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let len = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.split_text.lock().await.len()
        })
    });
    FnOutput::Text(len.to_string())
}
