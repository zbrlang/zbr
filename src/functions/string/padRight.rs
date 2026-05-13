use crate::context::{DiscordContext, FnOutput};

/// ZpadRight{text;width;char?}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    let width: usize = match args.get(1).and_then(|s| s.parse().ok()) {
        Some(w) => w,
        None => return FnOutput::error("padRight", "width is required"),
    };
    let pad_char: char = args.get(2).and_then(|s| s.chars().next()).unwrap_or(' ');
    let char_count = text.chars().count();
    if char_count >= width {
        return FnOutput::Text(text);
    }
    let padding: String = std::iter::repeat(pad_char)
        .take(width - char_count)
        .collect();
    FnOutput::Text(format!("{}{}", text, padding))
}
