use crate::context::{DiscordContext, FnOutput, ModalFieldData};

/// ZaddTextInput{fieldID;label;style?;minLength?;maxLength?;required?;placeholder?}
/// Adds a text input field to the current modal.
/// style: short (default) or paragraph
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let field_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("addTextInput", "fieldID is required"),
    };
    let label = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("addTextInput", "label is required"),
    };
    let style = match args.get(2) {
        Some(s) if s == "paragraph" => "paragraph".to_string(),
        _ => "short".to_string(),
    };
    let min_length: Option<u32> = match args.get(3) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => Some(n),
            Err(_) => return FnOutput::error("addTextInput", format!("invalid minLength: '{}'", s)),
        },
        _ => None,
    };
    let max_length: Option<u32> = match args.get(4) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => Some(n),
            Err(_) => return FnOutput::error("addTextInput", format!("invalid maxLength: '{}'", s)),
        },
        _ => None,
    };
    let required = match args.get(5) {
        Some(s) if s == "false" => false,
        _ => true,
    };
    let placeholder = match args.get(6) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut state = ctx.components.lock().await;
            match &mut state.modal {
                Some(m) => {
                    m.fields.push(ModalFieldData {
                        field_id,
                        label,
                        style,
                        min_length,
                        max_length,
                        required,
                        placeholder,
                        value: None,
                    });
                    Ok(())
                }
                None => Err("no modal — call ZnewModal first"),
            }
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("addTextInput", e),
    }
}
