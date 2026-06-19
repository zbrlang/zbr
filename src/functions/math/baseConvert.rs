use crate::context::{DiscordContext, FnOutput};
use super::helpers::parse_i64;

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let number_str = &args[0];
    let from_base = match parse_i64(&args[1], "baseConvert", 2, "fromBase") {
        Ok(v) => v as u32, Err(e) => return e,
    };
    let to_base = match parse_i64(&args[2], "baseConvert", 3, "toBase") {
        Ok(v) => v as u32, Err(e) => return e,
    };

    if from_base < 2 || from_base > 36 || to_base < 2 || to_base > 36 {
        return FnOutput::error("baseConvert", "Bases must be between 2 and 36");
    }

    let val = match i64::from_str_radix(number_str, from_base) {
        Ok(v) => v,
        Err(_) => return FnOutput::error("baseConvert", format!("Invalid number for base {}: {}", from_base, number_str)),
    };

    FnOutput::Text(format_radix(val, to_base))
}

fn format_radix(mut x: i64, radix: u32) -> String {
    if x == 0 {
        return "0".to_string();
    }
    let mut result = String::new();
    let negative = x < 0;
    if negative {
        x = -x;
    }
    while x > 0 {
        let m = (x % radix as i64) as u32;
        result.insert(0, std::char::from_digit(m, radix).unwrap().to_ascii_uppercase());
        x /= radix as i64;
    }
    if negative {
        result.insert(0, '-');
    }
    result
}
