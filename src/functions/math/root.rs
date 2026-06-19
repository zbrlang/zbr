use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let number = match parse_f64(&args[0], "root", 1, "number") {
        Ok(v) => v, Err(e) => return e,
    };
    let degree = match parse_f64(&args[1], "root", 2, "degree") {
        Ok(v) => v, Err(e) => return e,
    };

    if degree == 0.0 {
        return FnOutput::error("root", "Degree cannot be zero");
    }

    let result = number.powf(1.0 / degree);
    FnOutput::Text(result.to_string())
}
