use crate::context::{DiscordContext, FnOutput};
use regex::Regex;

/// ZregexReplace{text;pattern;replacement}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let pattern = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("regexReplace", crate::error_messages::required(2, "pattern")),
    };
    let replacement = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    match Regex::new(&pattern) {
        Ok(re) => FnOutput::Text(re.replace_all(&text, replacement.as_str()).into_owned()),
        Err(e) => FnOutput::error("regexReplace", crate::error_messages::action_failed_reason("compile regex", &e.to_string())),
    }
}
