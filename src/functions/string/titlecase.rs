use crate::context::{DiscordContext, FnOutput};

/// Converts the first letter of each word to uppercase, rest to lowercase.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let result = args[0]
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    upper + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    FnOutput::Text(result)
}
