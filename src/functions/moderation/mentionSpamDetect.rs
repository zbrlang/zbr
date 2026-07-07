use crate::context::{ DiscordContext, FnOutput };
use regex::Regex;
use once_cell::sync::Lazy;

static MENTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<@!?\d+>|<@&\d+>|@everyone|@here").unwrap()
});

/// ZmentionSpamDetect{threshold?}
/// Counts mentions in the current message and checks against threshold.
/// Returns "true" if mention spam detected, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let threshold: usize = match args.get(0) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "mentionSpamDetect",
                        crate::error_messages::expected_integer(1, "threshold", s)
                    );
                }
            }
        _ => 5,
    };

    let message = ctx.message.clone();
    let mention_count = MENTION_REGEX.find_iter(&message).count();

    FnOutput::Text((if mention_count >= threshold { "true" } else { "false" }).to_string())
}
