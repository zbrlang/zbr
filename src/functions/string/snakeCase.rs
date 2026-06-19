use crate::context::{DiscordContext, FnOutput};

/// ZsnakeCase{text}
/// Converts text to snake_case.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let words = get_words(text);
    FnOutput::Text(words.join("_"))
}

fn get_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            current_word.push(c);
        } else if !current_word.is_empty() {
            words.push(current_word.to_lowercase());
            current_word = String::new();
        }
    }
    if !current_word.is_empty() {
        words.push(current_word.to_lowercase());
    }
    words
}
