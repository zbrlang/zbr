use crate::context::{DiscordContext, FnOutput};

/// ZremoveLinks{text} — removes all URLs (http/https) from the text.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    // Simple URL removal: strip tokens that start with http:// or https://
    let result = text
        .split_whitespace()
        .filter(|token| {
            let lower = token.to_lowercase();
            !lower.starts_with("http://") && !lower.starts_with("https://")
        })
        .collect::<Vec<_>>()
        .join(" ");
    FnOutput::Text(result)
}
