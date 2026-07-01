use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n1 = match parse_i64(&args[0], "bitWise", 1, "n1") {
        Ok(v) => v, Err(e) => return e,
    };
    
    let operation = match args.get(1) {
        Some(v) => v.to_uppercase(),
        None => return FnOutput::error("bitWise", crate::error_messages::too_few_args(2, 1)),
    };
    
    if operation == "NOT" {
        return FnOutput::Text((!n1).to_string());
    }

    if args.len() < 3 {
        return FnOutput::error("bitWise", crate::error_messages::too_few_args(3, args.len()));
    }

    let n2 = match parse_i64(&args[2], "bitWise", 3, "n2") {
        Ok(v) => v, Err(e) => return e,
    };

    let result = match operation.as_str() {
        "AND" => n1 & n2,
        "OR" => n1 | n2,
        "XOR" => n1 ^ n2,
        "LSHIFT" => n1 << n2,
        "RSHIFT" => n1 >> n2,
        _ => return FnOutput::error("bitWise", format!("Invalid operation: {}", operation)),
    };

    FnOutput::Text(result.to_string())
}
