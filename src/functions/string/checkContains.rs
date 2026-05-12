use crate::context::{DiscordContext, FnOutput};

/// ZcheckContains{text;word;...}
/// Returns "true" if the text contains any of the provided words (case-insensitive).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("checkContains", "text and at least one word are required");
    }
    let text = args[0].to_lowercase();
    let found = args[1..].iter().any(|w| text.contains(&w.to_lowercase()));
    FnOutput::Text(found.to_string())
}
