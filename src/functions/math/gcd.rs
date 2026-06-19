use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mut a = match parse_i64(&args[0], "gcd", 1, "a") {
        Ok(v) => v.abs(), Err(e) => return e,
    };
    let mut b = match parse_i64(&args[1], "gcd", 2, "b") {
        Ok(v) => v.abs(), Err(e) => return e,
    };

    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }

    FnOutput::Text(a.to_string())
}
