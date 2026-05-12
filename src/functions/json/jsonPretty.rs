use crate::context::{DiscordContext, FnOutput};
use super::helpers::with_json;
use serde::Serialize;

/// ZjsonPretty{indent?}
/// Serializes the working JSON object to a pretty-printed string.
/// indent defaults to 2 spaces. Returns empty string if no object is set.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let indent: usize = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("jsonPretty", format!("invalid indent: '{}'", s)),
        },
        _ => 2,
    };

    with_json(ctx, |obj| match obj {
        None => FnOutput::Text(String::new()),
        Some(v) => {
            let indent_str = " ".repeat(indent);
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_str.as_bytes());
            let mut buf = Vec::new();
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            if v.serialize(&mut ser).is_ok() {
                FnOutput::Text(String::from_utf8_lossy(&buf).to_string())
            } else {
                FnOutput::error("jsonPretty", "failed to serialize JSON")
            }
        }
    })
}
