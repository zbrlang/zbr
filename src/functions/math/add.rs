use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mut result = 0.0f64;
    for (i, arg) in args.iter().enumerate() {
        let n = match parse_f64(arg, "add", i + 1, "value") {
            Ok(v) => v, Err(e) => return e,
        };
        result += n;
    }
    FnOutput::Text(result.to_string())
}
