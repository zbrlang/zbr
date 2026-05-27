use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mut best = f64::INFINITY;
    for (i, arg) in args.iter().enumerate() {
        let n = match parse_f64(arg, "min", i + 1, "value") {
            Ok(v) => v, Err(e) => return e,
        };
        if n < best { best = n; }
    }
    let s = if best.fract() == 0.0 && best.abs() < 1e15 {
        format!("{}", best as i64)
    } else {
        format!("{}", best)
    };
    FnOutput::Text(s)
}
