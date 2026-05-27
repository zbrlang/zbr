use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(&args[0], "ceil", 1, "value") {
        Ok(v) => v, Err(e) => return e,
    };
    FnOutput::Text((n.ceil() as i64).to_string())
}
