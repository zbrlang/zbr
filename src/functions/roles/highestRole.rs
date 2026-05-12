use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_opt: Option<String> = match args.get(0) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };
    let author_id = ctx.author_id.clone();

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("highestRole", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("highestRole", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let roles = GuildId::new(gid).roles(&http).await.map_err(|e| e.to_string())?;
            
            // Default to author when no userID given
            let uid_str = uid_opt.unwrap_or(author_id);
            let uid: u64 = uid_str.parse().map_err(|_| "invalid user ID".to_string())?;
            let member = GuildId::new(gid).member(&http, UserId::new(uid)).await.map_err(|_| "user not found".to_string())?;
            
            let highest = member.roles.iter().filter_map(|rid| roles.get(rid)).max_by_key(|r| r.position);
            Ok::<String, String>(highest.map(|r| r.id.to_string()).unwrap_or_default())
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("highestRole", e),
    }
}
