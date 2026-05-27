use crate::context::{DiscordContext, FnOutput};

/// Zbase64Encode{text}
/// Encodes text to base64.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    if text.is_empty() {
        return FnOutput::error("base64Encode", crate::error_messages::required(1, "text"));
    }
    FnOutput::Text(base64_encode(&text))
}

fn base64_encode(input: &str) -> String {
    use base64::engine::Engine;
    let engine = base64::prelude::BASE64_STANDARD;
    engine.encode(input.as_bytes())
}
