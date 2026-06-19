use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n1 = match parse_i64(&args[0], "bitWise", 1, "n1") {
        Ok(v) => v, Err(e) => return e,
    };
    
    let operation = args[2].to_uppercase();
    
    if operation == "NOT" {
        return FnOutput::Text((!n1).to_string());
    }

    let n2 = match parse_i64(&args[1], "bitWise", 2, "n2") {
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
