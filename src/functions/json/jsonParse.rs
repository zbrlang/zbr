use crate::context::{DiscordContext, FnOutput};
use super::helpers::with_json;

/// ZjsonParse{jsonString}
/// Parses a JSON string into the working object for this execution.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let s = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("jsonParse", "JSON string is required"),
    };

    let value: serde_json::Value = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return FnOutput::error("jsonParse", format!("invalid JSON: {}", e)),
    };

    with_json(ctx, |obj| *obj = Some(value));
    FnOutput::Empty
}
