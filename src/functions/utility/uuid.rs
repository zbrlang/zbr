use crate::context::{DiscordContext, FnOutput};
use uuid::Uuid;

/// Zuuid{}
/// Returns a random v4 UUID.
pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(Uuid::new_v4().to_string())
}
