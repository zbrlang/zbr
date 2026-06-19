use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0).filter(|s| !s.is_empty()) {
        Some(s) => s.clone(),
        _ => ctx.author_id.clone(),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userLocale", crate::error_messages::expected_snowflake(1, "userID", &uid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userLocale", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            UserId::new(uid).to_user(&http).await
        })
    });

    match result {
        Ok(user) => FnOutput::Text(user.locale.clone().unwrap_or_else(|| "en-US".to_string())),
        Err(_) => FnOutput::error("userLocale", crate::error_messages::not_found("user", &uid_str)),
    }
}
