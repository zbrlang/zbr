use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

/// Zusername{userID?}
/// No args → returns the command author's username.
/// With userID → fetches and returns that user's username via HTTP.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::Text(ctx.username.clone()),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("username", crate::error_messages::expected_snowflake(1, "userID", &uid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("username", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            UserId::new(uid).to_user(&http).await
        })
    });

    match result {
        Ok(user) => FnOutput::Text(user.name.clone()),
        Err(_) => FnOutput::error("username", crate::error_messages::not_found("user", &uid_str)),
    }
}
