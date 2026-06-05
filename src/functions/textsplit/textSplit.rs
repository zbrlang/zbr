use crate::context::{DiscordContext, FnOutput};

/// ZtextSplit{text;separator}
/// Splits text by separator and stores the result in ctx.split_text.
/// If separator is empty, splits into individual characters.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let text      = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let separator = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let parts: Vec<String> = if separator.is_empty() {
        text.chars().map(|c| c.to_string()).collect()
    } else {
        text.split(separator.as_str()).map(|s| s.to_string()).collect()
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.split_text.lock().await = parts.clone();
        })
    });

    FnOutput::Empty
}
