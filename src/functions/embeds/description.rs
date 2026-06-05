use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "description") {
        Ok(i) => i, Err(e) => return e,
    };
    with_embed(ctx, index, |e| e.description = Some(text));
    FnOutput::Empty
}
