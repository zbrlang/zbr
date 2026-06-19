use crate::context::{DiscordContext, FnOutput};

/// ZfromCharCode{code}
/// Convert an integer code back to a character.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let code_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if code_str.is_empty() {
        return FnOutput::error("fromCharCode", crate::error_messages::required(1, "code"));
    }

    let code = match code_str.parse::<u32>() {
        Ok(c) => c,
        Err(_) => return FnOutput::error("fromCharCode", "Invalid character code."),
    };

    match std::char::from_u32(code) {
        Some(c) => FnOutput::Text(c.to_string()),
        None => FnOutput::error("fromCharCode", "Invalid Unicode scalar value."),
    }
}
