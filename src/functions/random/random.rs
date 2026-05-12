use crate::context::{DiscordContext, FnOutput};
use rand::Rng;

fn parse_i64(s: &str, pos: &str) -> Result<i64, FnOutput> {
    s.parse::<i64>().map_err(|_| FnOutput::error("random", format!("invalid integer for {}: '{}'", pos, s)))
}

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let min = match parse_i64(&args[0], "min") { Ok(v) => v, Err(e) => return e };
    let max = match parse_i64(&args[1], "max") { Ok(v) => v, Err(e) => return e };
    if min >= max {
        return FnOutput::error("random", format!("min ({}) must be less than max ({})", min, max));
    }
    FnOutput::Text(rand::thread_rng().gen_range(min..=max).to_string())
}
