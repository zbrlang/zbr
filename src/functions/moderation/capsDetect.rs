use crate::context::{ DiscordContext, FnOutput };

/// ZcapsDetect{message?;threshold?}
/// Detects excessive uppercase letters (caps lock spam).
/// Returns "true" if caps spam detected, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let message = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.message.clone(),
    };

    let threshold: f64 = match args.get(1) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n >= 0.0 && n <= 1.0 => n,
                _ => {
                    return FnOutput::error("capsDetect", "threshold must be between 0.0 and 1.0");
                }
            }
        _ => 0.7,
    };

    if message.is_empty() {
        return FnOutput::Text("false".to_string());
    }

    let letters: Vec<char> = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    if letters.is_empty() {
        return FnOutput::Text("false".to_string());
    }

    let uppercase_count = letters
        .iter()
        .filter(|c| c.is_uppercase())
        .count();
    let total_letters = letters.len();
    let uppercase_ratio = (uppercase_count as f64) / (total_letters as f64);

    FnOutput::Text((if uppercase_ratio >= threshold { "true" } else { "false" }).to_string())
}
