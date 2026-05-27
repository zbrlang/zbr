use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_mut_at_path};

/// ZjsonArraySort{key;...}
/// Sorts the array at the given key path in place (ascending, lexicographic for strings,
/// numeric for numbers).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonArraySort", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::error("jsonArraySort", crate::error_messages::requires_first("ZjsonParse or ZjsonArray")),
        Some(root) => match get_mut_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                arr.sort_by(|a, b| {
                    // Numbers sort numerically, everything else sorts by string representation
                    match (a.as_f64(), b.as_f64()) {
                        (Some(fa), Some(fb)) => fa.partial_cmp(&fb).unwrap_or(std::cmp::Ordering::Equal),
                        _ => a.to_string().cmp(&b.to_string()),
                    }
                });
                FnOutput::Empty
            }
            Some(_) => FnOutput::error("jsonArraySort", "target is not an array"),
            None => FnOutput::error("jsonArraySort", "key path not found"),
        },
    })
}
