use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_f64, parse_i64};

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(&args[0], "truncate", 1, "number") {
        Ok(v) => v, Err(e) => return e,
    };
    let decimals = match parse_i64(&args[1], "truncate", 2, "decimals") {
        Ok(v) => v, Err(e) => return e,
    };

    let factor = 10.0f64.powi(decimals as i32);
    let result = (n * factor).trunc() / factor;
    
    FnOutput::Text(result.to_string())
}
