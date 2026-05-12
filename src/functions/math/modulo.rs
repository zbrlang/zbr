use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let a = match parse_f64(&args[0], "modulo", "argument 1") {
        Ok(v) => v, Err(e) => return e,
    };
    let b = match parse_f64(&args[1], "modulo", "argument 2") {
        Ok(v) => v, Err(e) => return e,
    };
    if b == 0.0 {
        return FnOutput::error("modulo", "cannot modulo by zero");
    }
    let result = a % b;
    let s = if result.fract() == 0.0 && result.abs() < 1e15 {
        format!("{}", result as i64)
    } else {
        format!("{}", result)
    };
    FnOutput::Text(s)
}
