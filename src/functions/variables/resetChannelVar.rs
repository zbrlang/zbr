use crate::context::{DiscordContext, FnOutput};

// ZresetChannelVar{name} — deletes this var across every channel
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("resetChannelVar", "variable name is required"),
    };
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("resetChannelVar", "no database available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::reset_channel_var(&db, &bot_id, &name).await
        })
    });
    FnOutput::Empty
}
