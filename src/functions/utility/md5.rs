use crate::context::{DiscordContext, FnOutput};
use md5::{Digest, Md5};

/// Zmd5{text}
/// Returns the MD5 hash of the input text in hexadecimal.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if text.is_empty() {
        return FnOutput::error("md5", crate::error_messages::required(1, "text"));
    }
    let hash = Md5::digest(text.as_bytes());
    FnOutput::Text(format!("{:x}", hash))
}
