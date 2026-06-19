use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let a = match parse_f64(&args[0], "hypot", 1, "a") {
        Ok(v) => v, Err(e) => return e,
    };
    let b = match parse_f64(&args[1], "hypot", 2, "b") {
        Ok(v) => v, Err(e) => return e,
    };

    let result = a.hypot(b);
    FnOutput::Text(result.to_string())
}
