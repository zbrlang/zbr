// ZtryRun is handled as a special case in Runtime::evaluate (lazy evaluation).
// This stub is never called — the runtime intercepts "tryRun" before resolving args.
use crate::context::{DiscordContext, FnOutput};
pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    FnOutput::error("tryRun", crate::error_messages::internal_error("ZtryRun should be handled by the runtime"))
}
