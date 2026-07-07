use crate::context::{ DiscordContext, FnOutput };
use regex::Regex;
use once_cell::sync::Lazy;

static CUSTOM_EMOJI_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r"<a?:\w+:\d+>").unwrap() });

static UNICODE_EMOJI_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"[\u{1F600}-\u{1F64F}\u{1F300}-\u{1F5FF}\u{1F680}-\u{1F6FF}\u{1F700}-\u{1F77F}\u{1F780}-\u{1F7FF}\u{1F800}-\u{1F8FF}\u{1F900}-\u{1F9FF}\u{1FA00}-\u{1FA6F}\u{1FA70}-\u{1FAFF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}\u{FE00}-\u{FE0F}\u{1F1E6}-\u{1F1FF}]"
    ).unwrap()
});

/// ZemojiSpamDetect{message?;threshold?}
/// Detects excessive emoji usage in messages.
/// Returns "true" if emoji spam detected, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let message = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.message.clone(),
    };

    let threshold: usize = match args.get(1) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "emojiSpamDetect",
                        crate::error_messages::expected_integer(2, "threshold", s)
                    );
                }
            }
        _ => 10,
    };

    if message.is_empty() {
        return FnOutput::Text("false".to_string());
    }

    let custom_emoji_count = CUSTOM_EMOJI_REGEX.find_iter(&message).count();

    let unicode_emoji_count = UNICODE_EMOJI_REGEX.find_iter(&message).count();

    let total_emojis = custom_emoji_count + unicode_emoji_count;

    FnOutput::Text((if total_emojis >= threshold { "true" } else { "false" }).to_string())
}
