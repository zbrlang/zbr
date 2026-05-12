use crate::context::{DiscordContext, FnOutput};

/// ZjoinSplitText{separator}
/// Joins all split elements with the given separator.
/// If separator is empty, joins with no separator.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).cloned().unwrap_or_default();

    let parts = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.split_text.lock().await.clone()
        })
    });

    FnOutput::Text(parts.join(&separator))
}
