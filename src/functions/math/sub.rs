use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

// First arg is the starting value, subsequent args are subtracted from it.
// Zsub{10;3;2} = 10 - 3 - 2 = 5
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mut iter = args.iter().enumerate();
    let (_, first) = iter.next().unwrap();
    let mut result = match parse_f64(first, "sub", 1, "value") {
        Ok(v) => v, Err(e) => return e,
    };
    for (i, arg) in iter {
        let n = match parse_f64(arg, "sub", i + 1, "value") {
            Ok(v) => v, Err(e) => return e,
        };
        result -= n;
    }
    FnOutput::Text(result.to_string())
}
