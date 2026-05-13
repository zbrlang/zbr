use super::helpers::parse_f64;
use crate::context::{DiscordContext, FnOutput};

/// Zround{value;decimals?}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(
        args.get(0).map(|s| s.as_str()).unwrap_or(""),
        "round",
        "argument 1",
    ) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let decimals: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let factor = 10f64.powi(decimals as i32);
    let result = (n * factor).round() / factor;
    if decimals == 0 {
        FnOutput::Text(format!("{}", result as i64))
    } else {
        FnOutput::Text(format!("{:.prec$}", result, prec = decimals as usize))
    }
}
