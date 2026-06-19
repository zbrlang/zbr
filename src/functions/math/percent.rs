use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let part = match parse_f64(&args[0], "percent", 1, "part") {
        Ok(v) => v, Err(e) => return e,
    };
    let total = match parse_f64(&args[1], "percent", 2, "total") {
        Ok(v) => v, Err(e) => return e,
    };
    
    if total == 0.0 {
        return FnOutput::error("percent", "Total cannot be zero");
    }

    let decimals = if args.len() > 2 {
        match args[2].parse::<i32>() {
            Ok(d) => Some(d),
            Err(_) => return FnOutput::error("percent", format!("Invalid decimals: {}", args[2])),
        }
    } else {
        None
    };

    let result = (part / total) * 100.0;
    
    let output = match decimals {
        Some(d) => format!("{:.1$}", result, d as usize),
        None => result.to_string(),
    };

    FnOutput::Text(output)
}
