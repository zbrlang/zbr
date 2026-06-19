use crate::context::{DiscordContext, FnOutput};

/// ZisEven{n}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let n: i64 = match args.get(0).and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return FnOutput::error("isEven", crate::error_messages::required(1, "number")),
    };
    FnOutput::Text((n % 2 == 0).to_string())
}
