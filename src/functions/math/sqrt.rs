use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(&args[0], "sqrt", 1, "value") {
        Ok(v) => v, Err(e) => return e,
    };
    if n < 0.0 {
        return FnOutput::error("sqrt", crate::error_messages::action_failed_reason("take square root", &format!("input {} is negative", n)));
    }
    let result = n.sqrt();
    let s = if result.fract() == 0.0 && result.abs() < 1e15 {
        format!("{}", result as i64)
    } else {
        format!("{}", result)
    };
    FnOutput::Text(s)
}
