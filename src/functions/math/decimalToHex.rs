use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_i64(&args[0], "decimalToHex", 1, "number") {
        Ok(v) => v, Err(e) => return e,
    };

    FnOutput::Text(format!("{:X}", n))
}
