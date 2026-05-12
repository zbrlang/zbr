use crate::context::{DiscordContext, FnOutput};

/// ZallowUserMentions{userID;...}
/// 0 args  → no user pings allowed on this response
/// 1+ args → only the listed user IDs may be pinged
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let ids: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.allowed_user_mentions.lock().await = Some(ids);
        })
    });

    FnOutput::Empty
}
