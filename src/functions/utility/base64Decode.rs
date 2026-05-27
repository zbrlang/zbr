use crate::context::{DiscordContext, FnOutput};

/// Zbase64Decode{text}
/// Decodes base64 to text.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    if text.is_empty() {
        return FnOutput::error("base64Decode", crate::error_messages::required(1, "base64 text"));
    }

    match base64_decode(&text) {
        Ok(decoded) => FnOutput::Text(decoded),
        Err(e) => FnOutput::error("base64Decode", e),
    }
}

fn base64_decode(input: &str) -> Result<String, String> {
    use base64::engine::Engine;
    let engine = base64::prelude::BASE64_STANDARD;
    let bytes = engine
        .decode(input)
        .map_err(|e| format!("invalid base64: {}", e))?;
    String::from_utf8(bytes).map_err(|e| format!("invalid UTF-8: {}", e))
}
