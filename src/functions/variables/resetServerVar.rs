use crate::context::{DiscordContext, FnOutput};

// ZresetServerVar{name} — deletes this var across every server
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("resetServerVar", crate::error_messages::required(1, "name")),
    };
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("resetServerVar", "no database available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::reset_server_var(&db, &bot_id, &name).await
        })
    });
    FnOutput::Empty
}
