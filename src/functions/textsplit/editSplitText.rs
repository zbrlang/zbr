use crate::context::{DiscordContext, FnOutput};

/// ZeditSplitText{index;value}
/// Replaces the element at the given 1-based index with a new value.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index_str = args.get(0).cloned().unwrap_or_default();
    let value     = args.get(1).cloned().unwrap_or_default();

    let index: usize = match index_str.parse::<usize>() {
        Ok(i) if i > 0 => i - 1, // convert to 0-based
        Ok(i) if i == 0 => return FnOutput::error("editSplitText", crate::error_messages::must_be_positive(1, "index", 0)),
        _ => return FnOutput::error("editSplitText", crate::error_messages::expected_integer(1, "index", &index_str)),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut parts = ctx.split_text.lock().await;
            if index >= parts.len() {
                return Err(crate::error_messages::out_of_range(1, "index", 1, parts.len() as i64, (index + 1) as i64));
            }
            parts[index] = value;
            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editSplitText", e),
    }
}
