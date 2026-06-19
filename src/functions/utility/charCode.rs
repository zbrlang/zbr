use crate::context::{DiscordContext, FnOutput};

/// ZcharCode{char}
/// Get the Unicode scalar value as an integer.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if text.is_empty() {
        return FnOutput::error("charCode", crate::error_messages::required(1, "char"));
    }

    let c = text.chars().next().unwrap();
    FnOutput::Text((c as u32).to_string())
}
