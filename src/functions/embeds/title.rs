use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "title") {
        Ok(i) => i, Err(e) => return e,
    };
    with_embed(ctx, index, |e| e.title = Some(text));
    FnOutput::Empty
}
