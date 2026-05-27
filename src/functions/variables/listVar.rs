use crate::context::{DiscordContext, FnOutput};

// ZlistVar{separator}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).cloned().unwrap_or_else(|| ", ".to_string());
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("listVar", crate::error_messages::not_available("database")),
    };
    let names = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::list_global_vars(&db, &bot_id).await
        })
    });
    FnOutput::Text(names.join(&separator))
}
