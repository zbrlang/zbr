use super::helpers::parse_f64;
use crate::context::{DiscordContext, FnOutput};

/// Zabs{value}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(
        args.get(0).map(|s| s.as_str()).unwrap_or(""),
        "abs",
        "argument 1",
    ) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let result = n.abs();
    if result.fract() == 0.0 {
        FnOutput::Text(format!("{}", result as i64))
    } else {
        FnOutput::Text(result.to_string())
    }
}
