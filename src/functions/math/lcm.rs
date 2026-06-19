use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let a = match parse_i64(&args[0], "lcm", 1, "a") {
        Ok(v) => v.abs(), Err(e) => return e,
    };
    let b = match parse_i64(&args[1], "lcm", 2, "b") {
        Ok(v) => v.abs(), Err(e) => return e,
    };

    if a == 0 || b == 0 {
        return FnOutput::Text("0".to_string());
    }

    let gcd = {
        let mut x = a;
        let mut y = b;
        while y != 0 {
            x %= y;
            std::mem::swap(&mut x, &mut y);
        }
        x
    };

    let result = (a / gcd) * b;
    FnOutput::Text(result.to_string())
}
