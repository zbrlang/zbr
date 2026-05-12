use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index = match parse_index(args.get(0), "timestamp") {
        Ok(i) => i, Err(e) => return e,
    };
    with_embed(ctx, index, |e| e.timestamp = true);
    FnOutput::Empty
}
