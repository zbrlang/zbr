use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, validate_url, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "thumbnail") {
        Ok(i) => i, Err(e) => return e,
    };
    if let Err(e) = validate_url(&url, "thumbnail") { return e; }
    with_embed(ctx, index, |e| e.thumbnail = Some(url));
    FnOutput::Empty
}
