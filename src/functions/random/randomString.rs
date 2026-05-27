use crate::context::{DiscordContext, FnOutput};
use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// ZrandomString{length} — generates a random alphanumeric string.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let len: usize = match args[0].parse() {
        Ok(n) if n > 0 && n <= 2000 => n,
        Ok(0) => return FnOutput::error("randomString", crate::error_messages::must_be_positive(1, "length", 0)),
        _ => return FnOutput::error("randomString", format!("invalid length: '{}'", args[0])),
    };
    let mut rng = rand::thread_rng();
    let result: String = (0..len)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect();
    FnOutput::Text(result)
}
