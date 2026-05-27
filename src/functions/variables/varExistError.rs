use crate::context::{DiscordContext, FnOutput};

/// ZvarExistError{name;error}
/// Halts with the error message if the global variable does not exist.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("varExistError", crate::error_messages::required(1, "name")),
    };
    let error_msg = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("varExistError", crate::error_messages::required(2, "error")),
    };

    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("varExistError", "no database available"),
    };

    let exists = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::global_var_exists(&db, &bot_id, &name).await
        })
    });

    if exists {
        FnOutput::Empty
    } else {
        FnOutput::UserError(error_msg)
    }
}
