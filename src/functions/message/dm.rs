use crate::context::{DiscordContext, FnOutput};
use serenity::builder::CreateMessage;
use serenity::model::id::UserId;

/// Zdm{userID;content}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("dm", crate::error_messages::required(1, "userID")),
    };
    let content = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("dm", crate::error_messages::required(2, "content")),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("dm", crate::error_messages::expected_snowflake(1, "userID", &uid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("dm", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let user = UserId::new(uid).to_user(&http).await.map_err(|_| "user not found".to_string())?;
            let dm = user.create_dm_channel(&http).await.map_err(|_| "failed to open DM channel".to_string())?;
            dm.send_message(&http, CreateMessage::new().content(content))
                .await
                .map_err(|_| "failed to send DM".to_string())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("dm", e),
    }
}
