use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_i64(&args[0], "factorial", 1, "n") {
        Ok(v) => v, Err(e) => return e,
    };

    if n < 0 {
        return FnOutput::error("factorial", "Number must be non-negative");
    }

    if n > 20 {
        return FnOutput::error("factorial", "Number too large (max 20)");
    }

    let mut result: i64 = 1;
    for i in 1..=n {
        result *= i;
    }

    FnOutput::Text(result.to_string())
}
