use crate::context::{DiscordContext, FnOutput};

/// Zsubstring{text;start;length?}
/// start is 1-based. length defaults to rest of string.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    let start: usize = match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
        Some(n) if n >= 1 => n - 1,
        Some(_) => return FnOutput::error("substring", crate::error_messages::must_be_positive(2, "start", 0)),
        None => return FnOutput::error("substring", crate::error_messages::required(2, "start")),
    };
    let chars: Vec<char> = text.chars().collect();
    if start >= chars.len() {
        return FnOutput::Text(String::new());
    }
    let length: usize = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(chars.len() - start);
    let end = (start + length).min(chars.len());
    FnOutput::Text(chars[start..end].iter().collect())
}
