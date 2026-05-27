use crate::context::{DiscordContext, FnOutput};
use super::helpers::with_json;

/// ZjsonParse{jsonString}
/// Parses a JSON string into the working object for this execution.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let s = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("jsonParse", crate::error_messages::required(1, "JSON string")),
    };

    let value: serde_json::Value = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return FnOutput::error("jsonParse", crate::error_messages::action_failed_reason("parse JSON", &e.to_string())),
    };

    with_json(ctx, |obj| *obj = Some(value));
    FnOutput::Empty
}
