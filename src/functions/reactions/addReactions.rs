use crate::context::{DiscordContext, FnOutput};

/// ZaddReactions{emoji1;emoji2;...}
/// Queues reactions to be added to the bot's own response after it is sent.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("addReactions", "at least one emoji is required");
    }
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut reactions = ctx.pending_reactions.lock().await;
            for emoji in &args {
                if !emoji.is_empty() {
                    reactions.push(emoji.clone());
                }
            }
        })
    });
    FnOutput::Empty
}
