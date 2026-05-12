use crate::context::{DiscordContext, FnOutput};

// ZresetUserVar{name} — deletes this var for every user in the current guild
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("resetUserVar", "variable name is required"),
    };
    let guild_id = ctx.guild_id.clone();
    let bot_id   = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("resetUserVar", "no database available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::reset_user_var(&db, &bot_id, &guild_id, &name).await
        })
    });
    FnOutput::Empty
}
