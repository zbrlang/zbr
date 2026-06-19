use crate::context::{DiscordContext, FnOutput};

/// ZhslToHex{h; s; l}
/// Converts HSL to hex.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("hslToHex", crate::error_messages::required(3, "h; s; l"));
    }

    let h = args[0].parse::<f64>();
    let s = args[1].parse::<f64>();
    let l = args[2].parse::<f64>();

    match (h, s, l) {
        (Ok(h), Ok(s), Ok(l)) => {
            let s = s / 100.0;
            let l = l / 100.0;

            let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
            let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
            let m = l - c / 2.0;

            let (r, g, b) = if h < 60.0 {
                (c, x, 0.0)
            } else if h < 120.0 {
                (x, c, 0.0)
            } else if h < 180.0 {
                (0.0, c, x)
            } else if h < 240.0 {
                (0.0, x, c)
            } else if h < 300.0 {
                (x, 0.0, c)
            } else {
                (c, 0.0, x)
            };

            let r = ((r + m) * 255.0).round() as u8;
            let g = ((g + m) * 255.0).round() as u8;
            let b = ((b + m) * 255.0).round() as u8;

            FnOutput::Text(format!("#{:02X}{:02X}{:02X}", r, g, b))
        }
        _ => FnOutput::error("hslToHex", "Invalid HSL values."),
    }
}
