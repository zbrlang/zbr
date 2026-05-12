use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

// First arg is the starting value, divided by each subsequent arg.
// Zdiv{100;5;2} = 100 / 5 / 2 = 10
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mut iter = args.iter().enumerate();
    let (_, first) = iter.next().unwrap();
    let mut result = match parse_f64(first, "div", "argument 1") {
        Ok(v) => v, Err(e) => return e,
    };
    for (i, arg) in iter {
        let n = match parse_f64(arg, "div", &format!("argument {}", i + 1)) {
            Ok(v) => v, Err(e) => return e,
        };
        if n == 0.0 {
            return FnOutput::error("div", "cannot divide by zero");
        }
        result /= n;
    }
    FnOutput::Text(result.to_string())
}
