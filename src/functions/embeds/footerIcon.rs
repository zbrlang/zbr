use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, read_embed, validate_url, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = args.get(0).cloned().unwrap_or_default();
    let index = match parse_index(args.get(1), "footerIcon") {
        Ok(i) => i, Err(e) => return e,
    };
    if let Err(e) = validate_url(&url, "footerIcon") { return e; }
    if read_embed(ctx, index, |e| e.footer.clone()).is_none() {
        return FnOutput::error("footerIcon", crate::error_messages::requires_set_first("footer"));
    }
    with_embed(ctx, index, |e| e.footer_icon = Some(url));
    FnOutput::Empty
}
