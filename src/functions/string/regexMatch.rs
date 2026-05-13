use crate::context::{DiscordContext, FnOutput};
use regex::Regex;

/// ZregexMatch{text;pattern}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    let pattern = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("regexMatch", "pattern is required"),
    };
    match Regex::new(&pattern) {
        Ok(re) => FnOutput::Text(re.is_match(&text).to_string()),
        Err(e) => FnOutput::error("regexMatch", format!("invalid pattern: {}", e)),
    }
}
