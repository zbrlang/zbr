use crate::context::{DiscordContext, FnOutput};

// ZgetChannelVar{name;(channelID)}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("getChannelVar", crate::error_messages::required(1, "name")),
    };
    let channel_id = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let bot_id     = ctx.bot_id.clone();
    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("getChannelVar", "no database available"),
    };
    
    let cache_key = format!("channel:{}:{}:{}", channel_id, name, bot_id);
    if let Ok(cache) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { Ok::<_, ()>(ctx.var_cache.lock().await.get(&cache_key).cloned()) })
    }) {
        if let Some(val) = cache {
            return FnOutput::Text(val);
        }
    }

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            crate::db::get_channel_var(&db, &bot_id, &channel_id, &name).await
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
