use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{ChannelId, UserId};

/// ZthreadAddMember{threadID;userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let thread_id = match validate_snowflake(args.get(0).unwrap_or(&String::new()), "threadAddMember", "thread ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let user_id = match validate_snowflake(args.get(1).unwrap_or(&String::new()), "threadAddMember", "user ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadAddMember", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(thread_id).add_thread_member(&http, UserId::new(user_id)).await
                .map_err(|e| crate::error_messages::action_failed_reason("add member", &e.to_string()))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("threadAddMember", e),
    }
}
