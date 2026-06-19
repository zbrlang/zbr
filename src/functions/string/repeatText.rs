use crate::context::{DiscordContext, FnOutput};
use crate::error_messages;

/// ZrepeatText{text; count}
/// Repeats `text` `count` times.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let count = match args[1].parse::<usize>() {
        Ok(c) => c,
        Err(_) => return FnOutput::error("repeatText", error_messages::expected_number(2, "count", &args[1])),
    };

    FnOutput::Text(text.repeat(count))
}
