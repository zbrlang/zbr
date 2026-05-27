use crate::context::{DiscordContext, FnOutput};
use sha2::Digest;
use std::fmt::Write;

/// Zsha256{text}
/// Returns the SHA-256 hash of the input text as a hex string.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    if text.is_empty() {
        return FnOutput::error("sha256", crate::error_messages::required(1, "text"));
    }
    let hash = sha2::Sha256::digest(text.as_bytes());
    let hex = hash.iter().fold(String::new(), |mut s, b| {
        write!(s, "{:02x}", b).unwrap();
        s
    });
    FnOutput::Text(hex)
}
