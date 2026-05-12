use crate::context::{DiscordContext, FnOutput};

// ZsetServerVar{name;value;(guildID)}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("setServerVar", "variable name is required"),
    };
    let value    = args.get(1).cloned().unwrap_or_default();
    let guild_id = args.get(2).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    let bot_id   = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("setServerVar", "no database available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::set_server_var(&db, &bot_id, &guild_id, &name, &value).await
        })
    });
    FnOutput::Empty
}
