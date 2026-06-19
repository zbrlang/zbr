use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_f64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n = match parse_f64(&args[0], "financial", 1, "number") {
        Ok(v) => v, Err(e) => return e,
    };
    let symbol = args.get(1).cloned().unwrap_or_else(|| "$".to_string());
    
    let sign = if n < 0.0 { "-" } else { "" };
    let abs_n = n.abs();
    
    let int_part = abs_n.trunc() as i64;
    let fract_part = ((abs_n - abs_n.trunc()) * 100.0).round() as i64;
    
    let mut int_str = int_part.to_string();
    let mut result = String::new();
    
    while int_str.len() > 3 {
        let last_three = int_str.split_off(int_str.len() - 3);
        result = format!(",{}{}", last_three, result);
    }
    result = format!("{}{}", int_str, result);
    
    FnOutput::Text(format!("{}{}{}.{:02}", sign, symbol, result, fract_part))
}
