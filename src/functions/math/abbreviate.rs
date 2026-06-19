use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(&args[0], "abbreviate", 1, "number") {
        Ok(v) => v, Err(e) => return e,
    };

    let abs_n = n.abs();
    let sign = if n < 0.0 { "-" } else { "" };

    let result = if abs_n >= 1_000_000_000_000.0 {
        format!("{sign}{:.1}T", abs_n / 1_000_000_000_000.0)
    } else if abs_n >= 1_000_000_000.0 {
        format!("{sign}{:.1}B", abs_n / 1_000_000_000.0)
    } else if abs_n >= 1_000_000.0 {
        format!("{sign}{:.1}M", abs_n / 1_000_000.0)
    } else if abs_n >= 1_000.0 {
        format!("{sign}{:.1}K", abs_n / 1_000.0)
    } else {
        n.to_string()
    };

    // Remove trailing .0 before suffix
    let result = result
        .replace(".0K", "K")
        .replace(".0M", "M")
        .replace(".0B", "B")
        .replace(".0T", "T");

    FnOutput::Text(result)
}
