use crate::context::{DiscordContext, FnOutput};

// ZsetVar{name;value}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("setVar", "variable name is required"),
    };
    let value  = args.get(1).cloned().unwrap_or_default();
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("setVar", "no database available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::set_global_var(&db, &bot_id, &name, &value).await
        })
    });
    FnOutput::Empty
}
