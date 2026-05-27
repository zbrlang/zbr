use crate::context::{DiscordContext, FnOutput};

// ZvarExists{name}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("varExists", crate::error_messages::required(1, "name")),
    };
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("varExists", "no database available"),
    };
    let exists = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::global_var_exists(&db, &bot_id, &name).await
        })
    });
    FnOutput::Text(exists.to_string())
}
