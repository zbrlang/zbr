use crate::context::{DiscordContext, FnOutput};

/// ZrgbToHex{r; g; b}
/// Converts R, G, B to #RRGGBB.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("rgbToHex", crate::error_messages::required(3, "r; g; b"));
    }

    let r = args[0].parse::<u8>();
    let g = args[1].parse::<u8>();
    let b = args[2].parse::<u8>();

    match (r, g, b) {
        (Ok(r), Ok(g), Ok(b)) => FnOutput::Text(format!("#{:02X}{:02X}{:02X}", r, g, b)),
        _ => FnOutput::error("rgbToHex", "Invalid RGB values. Expected integers between 0 and 255."),
    }
}
