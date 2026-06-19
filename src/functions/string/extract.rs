use crate::context::{DiscordContext, FnOutput};
use regex::Regex;

/// Zextract{text; type}
/// types: "urls", "emails", "mentions", "emojis", "numbers".
/// Use regex or simple parsing to find all matches and return them joined by newline.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let extract_type = args[1].to_lowercase();

    let pattern = match extract_type.as_str() {
        "urls" => r"https?://[^\s/$.?#].[^\s]*",
        "emails" => r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
        "mentions" => r"<@!?[0-9]+>|<@&[0-9]+>|<#[0-9]+>",
        "emojis" => r"<a?:\w+:[0-9]+>|[\u{1F300}-\u{1F9FF}]",
        "numbers" => r"[0-9]+",
        _ => return FnOutput::error("extract", format!("invalid type '{}'. Valid types: urls, emails, mentions, emojis, numbers", extract_type)),
    };

    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => return FnOutput::error("extract", format!("internal regex error: {}", e)),
    };
    
    let matches: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();

    FnOutput::Text(matches.join("\n"))
}
