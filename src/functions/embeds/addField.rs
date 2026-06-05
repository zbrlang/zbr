use crate::context::{DiscordContext, EmbedField, FnOutput};
use super::helpers::{parse_index, read_embed, with_embed};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let value = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let inline = args.get(2).map(|s| s == "true").unwrap_or(false);
    let index = match parse_index(args.get(3), "addField") {
        Ok(i) => i, Err(e) => return e,
    };

    if name.is_empty() {
        return FnOutput::error("addField", crate::error_messages::required(1, "name"));
    }
    if value.is_empty() {
        return FnOutput::error("addField", crate::error_messages::required(2, "value"));
    }

    let field_count = read_embed(ctx, index, |e| Some(e.fields.len())).unwrap_or(0);
    if field_count >= 25 {
        return FnOutput::error("addField", "embeds cannot have more than 25 fields");
    }

    with_embed(ctx, index, |e| e.fields.push(EmbedField { name, value, inline }));
    FnOutput::Empty
}
