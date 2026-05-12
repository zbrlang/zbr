use crate::context::{DiscordContext, FnOutput, SelectOptionData};

/// ZaddSelectMenuOption{label;value;description?;default?;emoji?}
/// Adds an option to the current select menu.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let label = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("addSelectMenuOption", "label is required"),
    };
    let value = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("addSelectMenuOption", "value is required"),
    };
    let description = match args.get(2) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };
    let default = match args.get(3) {
        Some(s) if s == "true" => true,
        _ => false,
    };
    let emoji = match args.get(4) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut state = ctx.components.lock().await;
            match &mut state.select_menu {
                Some(sm) => {
                    sm.options.push(SelectOptionData { label, value, description, emoji, default });
                    Ok(())
                }
                None => Err("no select menu — call ZnewSelectMenu first"),
            }
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("addSelectMenuOption", e),
    }
}
