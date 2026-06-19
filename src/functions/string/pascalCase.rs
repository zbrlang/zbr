use crate::context::{DiscordContext, FnOutput};

/// ZpascalCase{text}
/// Converts text to PascalCase.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let words = get_words(text);
    
    let result = words.iter()
        .map(|w| capitalize(w))
        .collect::<String>();
    
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
