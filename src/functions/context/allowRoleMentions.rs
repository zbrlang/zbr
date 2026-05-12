use crate::context::{DiscordContext, FnOutput};

/// ZallowRoleMentions{roleID;...}
/// 0 args  → no role pings allowed on this response
/// 1+ args → only the listed role IDs may be pinged
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let ids: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.allowed_role_mentions.lock().await = Some(ids);
        })
    });

    FnOutput::Empty
}
