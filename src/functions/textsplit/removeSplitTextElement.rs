use crate::context::{DiscordContext, FnOutput};

/// ZremoveSplitTextElement{index}
/// Removes the element at the given 1-based index from the split.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index_str = args.get(0).cloned().unwrap_or_default();

    let index: usize = match index_str.parse::<usize>() {
        Ok(i) if i > 0 => i - 1, // convert to 0-based
        Ok(0) => return FnOutput::error("removeSplitTextElement", "index must be 1 or greater"),
        _ => return FnOutput::error("removeSplitTextElement", format!("invalid index: '{}'", index_str)),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut parts = ctx.split_text.lock().await;
            if index >= parts.len() {
                return Err(format!("index {} is out of range (split has {} elements)", index + 1, parts.len()));
            }
            parts.remove(index);
            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("removeSplitTextElement", e),
    }
}
