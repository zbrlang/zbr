use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    if uid_str.is_empty() {
        return FnOutput::error("dmChannelID", crate::error_messages::required(1, "user ID"));
    }

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("dmChannelID", crate::error_messages::expected_snowflake(1, "user ID", &uid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("dmChannelID", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let user = UserId::new(uid).to_user(&http).await?;
            user.create_dm_channel(&http).await
        })
    });

    match result {
        Ok(ch) => FnOutput::Text(ch.id.to_string()),
        Err(_) => FnOutput::error("dmChannelID", crate::error_messages::action_failed("create DM channel")),
    }
}
