use crate::context::{DiscordContext, FnOutput, SelectMenuData};

/// ZnewSelectMenu{menuID;min?;max?;placeholder?}
/// Creates a string select menu on the response.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let menu_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("newSelectMenu", crate::error_messages::required(1, "menuID")),
    };
    let min_values: u8 = match args.get(1) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("newSelectMenu", crate::error_messages::expected_integer(2, "min", s)),
        },
        _ => 1,
    };
    let max_values: u8 = match args.get(2) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("newSelectMenu", crate::error_messages::expected_integer(3, "max", s)),
        },
        _ => 1,
    };
    let placeholder = match args.get(3) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.components.lock().await.select_menu = Some(SelectMenuData {
                menu_id,
                kind: "string".to_string(),
                min_values,
                max_values,
                placeholder,
                options: Vec::new(),
            });
        })
    });

    FnOutput::Empty
}
