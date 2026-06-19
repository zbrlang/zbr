use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let start = match parse_f64(&args[0], "lerp", 1, "start") {
        Ok(v) => v, Err(e) => return e,
    };
    let end = match parse_f64(&args[1], "lerp", 2, "end") {
        Ok(v) => v, Err(e) => return e,
    };
    let t = match parse_f64(&args[2], "lerp", 3, "t") {
        Ok(v) => v, Err(e) => return e,
    };

    let result = start + (end - start) * t;
    FnOutput::Text(result.to_string())
}
