use crate::context::{DiscordContext, FnOutput};

/// ZindexOf{text;search}
/// Returns 1-based position of first match, or 0 if not found.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).map(|s| s.as_str()).unwrap_or("");
    let search = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if search.is_empty() {
        return FnOutput::Text("0".to_string());
    }
    let pos = text
        .find(search)
        .map(|i| {
            text[..i].chars().count() + 1 // 1-based char index
        })
        .unwrap_or(0);
    FnOutput::Text(pos.to_string())
}
