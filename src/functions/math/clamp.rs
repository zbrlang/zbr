use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let value = match parse_f64(&args[0], "clamp", 1, "value") {
        Ok(v) => v, Err(e) => return e,
    };
    let min = match parse_f64(&args[1], "clamp", 2, "min") {
        Ok(v) => v, Err(e) => return e,
    };
    let max = match parse_f64(&args[2], "clamp", 3, "max") {
        Ok(v) => v, Err(e) => return e,
    };

    let result = value.clamp(min, max);
    FnOutput::Text(result.to_string())
}
