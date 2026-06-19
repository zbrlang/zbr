use crate::context::{DiscordContext, FnOutput};

/// ZcamelCase{text}
/// Converts text to camelCase.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let words = get_words(text);
    if words.is_empty() {
        return FnOutput::Text(String::new());
    }

    let mut result = words[0].clone();
    for word in words.iter().skip(1) {
        result.push_str(&capitalize(word));
    }
    
    FnOutput::Text(result)
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

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
