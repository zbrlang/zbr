use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, read_embed, validate_url, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "titleURL") {
        Ok(i) => i, Err(e) => return e,
    };
    if let Err(e) = validate_url(&url, "titleURL") { return e; }
    if read_embed(ctx, index, |e| e.title.clone()).is_none() {
        return FnOutput::error("titleURL", crate::error_messages::requires_set_first("title"));
    }
    with_embed(ctx, index, |e| e.title_url = Some(url));
    FnOutput::Empty
}
