use crate::context::{DiscordContext, FnOutput};

/// Zcensor{text; wordList; char?}
/// wordList: semicolon separated. Replace with char (default *).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let word_list: Vec<&str> = args[1].split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let pad_char = args.get(2).unwrap_or(&"*".to_string()).clone();

    let mut result = text.clone();
    for word in word_list {
        let replacement = pad_char.repeat(word.chars().count());
        result = result.replace(word, &replacement);
    }

    FnOutput::Text(result)
}
