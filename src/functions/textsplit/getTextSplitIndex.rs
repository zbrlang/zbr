use crate::context::{DiscordContext, FnOutput};

/// ZgetTextSplitIndex{value}
/// Returns the 1-based index of the first element matching value.
/// Returns -1 if not found.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let value = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let parts = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.split_text.lock().await.clone()
        })
    });

    let index = parts.iter().position(|s| s == &value)
        .map(|i| (i + 1) as i64)
        .unwrap_or(-1);

    FnOutput::Text(index.to_string())
}
