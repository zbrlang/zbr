use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(&args[0], "sqrt", "argument 1") {
        Ok(v) => v, Err(e) => return e,
    };
    if n < 0.0 {
        return FnOutput::error("sqrt", format!("cannot take square root of negative number: {}", n));
    }
    let result = n.sqrt();
    let s = if result.fract() == 0.0 && result.abs() < 1e15 {
        format!("{}", result as i64)
    } else {
        format!("{}", result)
    };
    FnOutput::Text(s)
}
