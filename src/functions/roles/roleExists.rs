use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let role_id_str = if args.get(0).map(|s| s.is_empty()).unwrap_or(true) {
        // fetch author's top role
        let http = ctx.http.as_ref().unwrap().clone();
        let guild_id = ctx.guild_id.parse::<u64>().map(GuildId::new).unwrap();
        let user_id = ctx.author_id.parse::<u64>().map(UserId::new).unwrap();
        let member = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async move { http.get_member(guild_id, user_id).await })
        });
        match member {
            Ok(m) => m.roles.first().map(|r| r.to_string()).unwrap_or_default(),
            Err(_) => return FnOutput::error("roleExists", "could not get author's top role"),
        }
    } else {
        args[0].clone()
    };

    let rid: u64 = match role_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::Text("false".to_string()),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::Text("false".to_string()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::Text("false".to_string()),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async move { GuildId::new(gid).roles(&http).await })
    });

    match result {
        Ok(roles) => {
            if roles.contains_key(&RoleId::new(rid)) {
                FnOutput::Text("true".to_string())
            } else {
                FnOutput::Text("false".to_string())
            }
        }
        Err(_) => FnOutput::Text("false".to_string()),
    }
}
