use crate::context::{DiscordContext, FnOutput};

/// ZhexToRgb{hex}
/// Converts hex to R, G, B.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let hex = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if hex.is_empty() {
        return FnOutput::error("hexToRgb", crate::error_messages::required(1, "hex"));
    }

    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return FnOutput::error("hexToRgb", "Invalid hex color format. Expected 6 characters.");
    }

    let r = u8::from_str_radix(&hex[0..2], 16);
    let g = u8::from_str_radix(&hex[2..4], 16);
    let b = u8::from_str_radix(&hex[4..6], 16);

    match (r, g, b) {
        (Ok(r), Ok(g), Ok(b)) => FnOutput::Text(format!("{}, {}, {}", r, g, b)),
        _ => FnOutput::error("hexToRgb", "Invalid hex color values."),
    }
}
