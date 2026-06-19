use crate::context::{DiscordContext, FnOutput};

/// Zpad{text; width; char?; side?}
/// width: integer, char: default space, side: "left", "right", or "center" (default "right").
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let width = match args[1].parse::<usize>() {
        Ok(w) => w,
        Err(_) => return FnOutput::error("pad", crate::error_messages::expected_integer(2, "width", &args[1])),
    };
    let pad_char = args.get(2).and_then(|s| s.chars().next()).unwrap_or(' ');
    let side = args.get(3).map(|s| s.to_lowercase()).unwrap_or_else(|| "right".to_string());

    let len = text.chars().count();
    if len >= width {
        return FnOutput::Text(text.clone());
    }

    let diff = width - len;
    let result = match side.as_str() {
        "left" => {
            let mut s = String::with_capacity(width);
            for _ in 0..diff { s.push(pad_char); }
            s.push_str(text);
            s
        }
        "center" => {
            let left_pad = diff / 2;
            let right_pad = diff - left_pad;
            let mut s = String::with_capacity(width);
            for _ in 0..left_pad { s.push(pad_char); }
            s.push_str(text);
            for _ in 0..right_pad { s.push(pad_char); }
            s
        }
        _ => { // right
            let mut s = String::with_capacity(width);
            s.push_str(text);
            for _ in 0..diff { s.push(pad_char); }
            s
        }
    };

    FnOutput::Text(result)
}
