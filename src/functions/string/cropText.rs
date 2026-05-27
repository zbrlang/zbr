use crate::context::{DiscordContext, FnOutput};

/// ZcropText{text;maxChars;ending}
/// Crops text to maxChars characters. If cropped, appends 'ending'.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text    = &args[0];
    let max_str = &args[1];
    let ending  = &args[2];

    let max: usize = match max_str.parse() {
        Ok(n) if n > 0 => n,
        _ => return FnOutput::error("cropText", crate::error_messages::expected_integer(2, "maxChars", max_str)),
    };

    let char_count = text.chars().count();
    if char_count <= max {
        return FnOutput::Text(text.clone());
    }

    // Crop to max - ending.len() chars so the ending fits within max
    let ending_len = ending.chars().count();
    let crop_to = if max > ending_len { max - ending_len } else { 0 };
    let cropped: String = text.chars().take(crop_to).collect();
    FnOutput::Text(format!("{}{}", cropped, ending))
}
