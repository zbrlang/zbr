use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, validate_color, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let hex = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "color") {
        Ok(i) => i, Err(e) => return e,
    };
    let color = match validate_color(&hex, "color") {
        Ok(c) => c, Err(e) => return e,
    };
    with_embed(ctx, index, |e| e.color = Some(color));
    FnOutput::Empty
}
