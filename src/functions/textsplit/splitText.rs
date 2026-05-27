use crate::context::{DiscordContext, FnOutput};

/// ZsplitText{index}
/// Returns one element from the split text by 1-based index.
/// Use "<" for the first element, ">" for the last.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index_str = args.get(0).cloned().unwrap_or_default();
    if index_str.is_empty() {
        return FnOutput::error("splitText", crate::error_messages::required(1, "index"));
    }

    let parts = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.split_text.lock().await.clone()
        })
    });

    if parts.is_empty() {
        return FnOutput::error("splitText", crate::error_messages::requires_first("ZtextSplit"));
    }

    let result = match index_str.as_str() {
        "<" => parts.first().cloned(),
        ">" => parts.last().cloned(),
        s => {
            match s.parse::<usize>() {
                Ok(i) if i > 0 && i <= parts.len() => parts.get(i - 1).cloned(),
                Ok(i) if i == 0 => return FnOutput::error("splitText", crate::error_messages::must_be_positive(1, "index", 0)),
                Ok(i) => return FnOutput::error("splitText", crate::error_messages::out_of_range(1, "index", 1, parts.len() as i64, i as i64)),
                Err(_) => return FnOutput::error("splitText", crate::error_messages::expected_choice(1, "index", "a number, '<', or '>'", s)),
            }
        }
    };

    match result {
        Some(v) => FnOutput::Text(v),
        None => FnOutput::Text(String::new()),
    }
}
