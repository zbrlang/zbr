use crate::context::{DiscordContext, FnOutput};

/// Zroman{number}
/// Convert integer to Roman Numeral (e.g., 14 -> XIV).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let num = match args[0].parse::<i64>() {
        Ok(n) => n,
        Err(_) => return FnOutput::error("roman", crate::error_messages::expected_integer(1, "number", &args[0])),
    };

    if num <= 0 || num > 3999 {
        return FnOutput::error("roman", "number must be between 1 and 3999");
    }

    let mut n = num;
    let mut roman = String::new();
    let values = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
    ];

    for (val, sym) in values {
        while n >= val {
            roman.push_str(sym);
            n -= val;
        }
    }

    FnOutput::Text(roman)
}
