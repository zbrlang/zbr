use crate::context::{DiscordContext, FnOutput};

// ZgetVar{name}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("getVar", "variable name is required"),
    };
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("getVar", "no database available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::get_global_var(&db, &bot_id, &name).await
        })
    });
    FnOutput::Text(result)
}
