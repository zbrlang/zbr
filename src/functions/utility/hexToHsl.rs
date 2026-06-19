use crate::context::{DiscordContext, FnOutput};

/// ZhexToHsl{hex}
/// Converts hex to H, S, L.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let hex = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if hex.is_empty() {
        return FnOutput::error("hexToHsl", crate::error_messages::required(1, "hex"));
    }

    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return FnOutput::error("hexToHsl", "Invalid hex color format. Expected 6 characters.");
    }

    let r = u8::from_str_radix(&hex[0..2], 16);
    let g = u8::from_str_radix(&hex[2..4], 16);
    let b = u8::from_str_radix(&hex[4..6], 16);

    match (r, g, b) {
        (Ok(r), Ok(g), Ok(b)) => {
            let r = r as f64 / 255.0;
            let g = g as f64 / 255.0;
            let b = b as f64 / 255.0;

            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let d = max - min;

            let mut h = if d == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / d) % 6.0
            } else if max == g {
                (b - r) / d + 2.0
            } else {
                (r - g) / d + 4.0
            };

            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }

            let l = (max + min) / 2.0;
            let s = if d == 0.0 {
                0.0
            } else {
                d / (1.0 - (2.0 * l - 1.0).abs())
            };

            FnOutput::Text(format!("{:.0}, {:.0}, {:.0}", h, s * 100.0, l * 100.0))
        }
        _ => FnOutput::error("hexToHsl", "Invalid hex color values."),
    }
}
