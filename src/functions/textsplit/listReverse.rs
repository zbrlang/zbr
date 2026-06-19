use crate::context::{DiscordContext, FnOutput};

/// ZlistReverse{list;separator} — reverses a list.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("listReverse", "expected 2 arguments: list;separator".to_string());
    }
    let separator = &args[1];
    let mut items: Vec<&str> = args[0].split(separator).collect();
    items.reverse();
    FnOutput::Text(items.join(separator))
}
