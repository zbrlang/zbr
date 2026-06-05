use crate::context::{ButtonData, DiscordContext, FnOutput};

/// ZaddButton{newRow;customID;label;style;disabled?;emoji?}
/// Adds a button to the bot's response.
/// style: primary, secondary, success, danger, link
/// newRow: "true" starts a new action row
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let new_row = match args.get(0) {
        Some(s) if !s.is_empty() => s == "true",
        _ => false,
    };
    let custom_id = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("addButton", crate::error_messages::required(2, "customID")),
    };
    let label = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let style = match args.get(3) {
        Some(s) if !s.is_empty() => s.to_lowercase(),
        _ => "secondary".to_string(),
    };

    let valid_styles = ["primary", "secondary", "success", "danger", "link"];
    if !valid_styles.contains(&style.as_str()) {
        return FnOutput::error("addButton", crate::error_messages::expected_choice(4, "style", "primary, secondary, success, danger, link", &style));
    }

    let disabled = match args.get(4) {
        Some(s) if s == "true" => true,
        _ => false,
    };
    let emoji = match args.get(5) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.components.lock().await.buttons.push(ButtonData {
                custom_id,
                label,
                style,
                disabled,
                emoji,
                new_row,
            });
        })
    });

    FnOutput::Empty
}
