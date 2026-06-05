use crate::context::{DiscordContext, FnOutput};
use regex::Regex;

/// ZregexMatch{text;pattern}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let pattern = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("regexMatch", crate::error_messages::required(2, "pattern")),
    };
    match Regex::new(&pattern) {
        Ok(re) => FnOutput::Text(re.is_match(&text).to_string()),
        Err(e) => FnOutput::error("regexMatch", crate::error_messages::action_failed_reason("compile regex", &e.to_string())),
    }
}
