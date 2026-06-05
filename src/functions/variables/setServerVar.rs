use crate::context::{DiscordContext, FnOutput};

// ZsetServerVar{name;value;(guildID)}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("setServerVar", crate::error_messages::required(1, "name")),
    };
    let value    = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let guild_id = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    let bot_id   = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("setServerVar", "no database available"),
    };
    
    let name_clone = name.clone();
    let value_clone = value.clone();
    let guild_id_clone = guild_id.clone();
    let db_clone = db.clone();
    let bot_id_clone = bot_id.clone();

    let res = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::set_server_var(&db_clone, &bot_id_clone, &guild_id_clone, &name_clone, &value_clone).await
        })
    });
    if let Err(e) = res {
        return FnOutput::error("setServerVar", crate::error_messages::action_failed_reason("set server variable", &e.to_string()));
    }
    
    let cache_key = format!("server:{}:{}:{}", guild_id, name, bot_id);
    let _ = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.var_cache.lock().await.insert(cache_key, value);
            Ok::<_, ()>(())
        })
    });

    FnOutput::Empty
}
