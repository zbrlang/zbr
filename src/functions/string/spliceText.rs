use crate::context::{DiscordContext, FnOutput};

/// ZspliceText{text;start;length;replacement}
/// text → original string
/// start → position to begin replacing (1-based)
/// length → how many characters to remove
/// replacement → text inserted in its place
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 4 {
        return FnOutput::error("spliceText", crate::error_messages::too_few_args(4, args.len()));
    }

    let text = &args[0];
    let chars: Vec<char> = text.chars().collect();
    let text_len = chars.len();

    let start: usize = match args[1].parse::<usize>() {
        Ok(n) if n >= 1 => (n - 1).min(text_len),
        Ok(_) => 0,
        _ => {
            let val = args[1].parse::<i64>().unwrap_or(0);
            return FnOutput::error("spliceText", crate::error_messages::must_be_positive(2, "start", val));
        }
    };

    let length: usize = match args[2].parse::<usize>() {
        Ok(n) => n,
        _ => {
            return FnOutput::error("spliceText", crate::error_messages::expected_number(3, "length", &args[2]));
        }
    };

    let replacement = &args[3];
    let end = (start + length).min(text_len);

    let mut result = String::with_capacity(text.len() + replacement.len());
    result.push_str(&chars[0..start].iter().collect::<String>());
    result.push_str(replacement);
    if end < text_len {
        result.push_str(&chars[end..].iter().collect::<String>());
    }

    FnOutput::Text(result)
}
