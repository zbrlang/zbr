use super::helpers::parse_f64;
use crate::context::{DiscordContext, FnOutput};

/// Zpow{base;exp}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let base = match parse_f64(args.get(0).map(|s| s.as_str()).unwrap_or(""), "pow", "base") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let exp = match parse_f64(
        args.get(1).map(|s| s.as_str()).unwrap_or(""),
        "pow",
        "exponent",
    ) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let result = base.powf(exp);
    if result.fract() == 0.0 && result.abs() < 1e15 {
        FnOutput::Text(format!("{}", result as i64))
    } else {
        FnOutput::Text(result.to_string())
    }
}
