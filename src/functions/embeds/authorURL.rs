use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, read_embed, validate_url, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = args.get(0).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "authorURL") {
        Ok(i) => i, Err(e) => return e,
    };
    if let Err(e) = validate_url(&url, "authorURL") { return e; }
    if read_embed(ctx, index, |e| e.author.clone()).is_none() {
        return FnOutput::error("authorURL", crate::error_messages::requires_set_first("author"));
    }
    with_embed(ctx, index, |e| e.author_url = Some(url));
    FnOutput::Empty
}
