use crate::context::{DiscordContext, FnOutput};

/// ZcolorInvert{hex}
/// Inverts the RGB values of a hex color.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let hex = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if hex.is_empty() {
        return FnOutput::error("colorInvert", crate::error_messages::required(1, "hex"));
    }

    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return FnOutput::error("colorInvert", "Invalid hex color format. Expected 6 characters.");
    }

    let r = u8::from_str_radix(&hex[0..2], 16);
    let g = u8::from_str_radix(&hex[2..4], 16);
    let b = u8::from_str_radix(&hex[4..6], 16);

    match (r, g, b) {
        (Ok(r), Ok(g), Ok(b)) => {
            FnOutput::Text(format!("#{:02X}{:02X}{:02X}", 255 - r, 255 - g, 255 - b))
        }
        _ => FnOutput::error("colorInvert", "Invalid hex color values."),
    }
}
