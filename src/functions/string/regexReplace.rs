use crate::context::{DiscordContext, FnOutput};
use regex::Regex;

/// ZregexReplace{text;pattern;replacement}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    let pattern = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("regexReplace", "pattern is required"),
    };
    let replacement = args.get(2).cloned().unwrap_or_default();
    match Regex::new(&pattern) {
        Ok(re) => FnOutput::Text(re.replace_all(&text, replacement.as_str()).into_owned()),
        Err(e) => FnOutput::error("regexReplace", format!("invalid pattern: {}", e)),
    }
}
