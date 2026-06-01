use crate::context::{DiscordContext, FnOutput};

// ZgetVar{name}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("getVar", crate::error_messages::required(1, "name")),
    };
    let bot_id = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("getVar", "no database available"),
    };
    
    let cache_key = format!("global:{}:{}", name, bot_id);
    if let Ok(cache) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { Ok::<_, ()>(ctx.var_cache.lock().await.get(&cache_key).cloned()) })
    }) {
        if let Some(val) = cache {
            return FnOutput::Text(val);
        }
    }

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::get_global_var(&db, &bot_id, &name).await
        })
    });

    let _ = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.var_cache.lock().await.insert(cache_key, result.clone());
            Ok::<_, ()>(())
        })
    });
    FnOutput::Text(result)
}
