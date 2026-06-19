use crate::context::{DiscordContext, FnOutput};

/// Zclean{text}
/// Prevent mentions by inserting a zero-width space (U+200B) after "@" and "#".
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let result = text.replace('@', "@\u{200B}").replace('#', "#\u{200B}");
    FnOutput::Text(result)
}
