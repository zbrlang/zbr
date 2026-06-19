use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let hex = args[0].trim().trim_start_matches("0x").trim_start_matches('#');
    
    match i64::from_str_radix(hex, 16) {
        Ok(v) => FnOutput::Text(v.to_string()),
        Err(_) => FnOutput::error("hexToDecimal", format!("Invalid hex string: {}", args[0])),
    }
}
