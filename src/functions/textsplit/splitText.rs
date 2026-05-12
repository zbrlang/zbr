use crate::context::{DiscordContext, FnOutput};

/// ZsplitText{index}
/// Returns one element from the split text by 1-based index.
/// Use "<" for the first element, ">" for the last.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index_str = args.get(0).cloned().unwrap_or_default();
    if index_str.is_empty() {
        return FnOutput::error("splitText", "index is required");
    }

    let parts = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.split_text.lock().await.clone()
        })
    });

    if parts.is_empty() {
        return FnOutput::error("splitText", "no split text available — call ZtextSplit first");
    }

    let result = match index_str.as_str() {
        "<" => parts.first().cloned(),
        ">" => parts.last().cloned(),
        s => {
            match s.parse::<usize>() {
                Ok(i) if i > 0 && i <= parts.len() => parts.get(i - 1).cloned(),
                Ok(0) => return FnOutput::error("splitText", "index must be 1 or greater"),
                Ok(_) => return FnOutput::error("splitText", format!("index {} is out of range (split has {} elements)", s, parts.len())),
                Err(_) => return FnOutput::error("splitText", format!("invalid index: '{}' (use a number, '<', or '>')", s)),
            }
        }
    };

    match result {
        Some(v) => FnOutput::Text(v),
        None => FnOutput::Text(String::new()),
    }
}
