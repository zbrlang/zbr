use crate::context::{DiscordContext, FnOutput};

/// ZnumberSeparator{number;(separator)}
/// Formats a number with thousands separators. Default separator is ','.
/// e.g. ZnumberSeparator{1000000} → 1,000,000
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let input = args[0].trim();
    let sep   = args.get(1).map(|s| s.as_str()).unwrap_or(",");

    // Split on decimal point to handle floats
    let (int_part, dec_part) = match input.find('.') {
        Some(pos) => (&input[..pos], Some(&input[pos..])),
        None      => (input, None),
    };

    // Handle negative sign
    let (sign, digits) = if int_part.starts_with('-') {
        ("-", &int_part[1..])
    } else {
        ("", int_part)
    };

    // Insert separators every 3 digits from the right
    let chars: Vec<char> = digits.chars().collect();
    let mut formatted = String::new();
    for (i, ch) in chars.iter().enumerate() {
        let remaining = chars.len() - i;
        if i > 0 && remaining % 3 == 0 {
            formatted.push_str(sep);
        }
        formatted.push(*ch);
    }

    let mut result = format!("{}{}", sign, formatted);
    if let Some(dec) = dec_part {
        result.push_str(dec);
    }

    FnOutput::Text(result)
}
