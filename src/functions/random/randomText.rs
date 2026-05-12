use crate::context::{DiscordContext, FnOutput};
use rand::seq::SliceRandom;

/// ZrandomText{text1;text2;...} — picks one value randomly.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("randomText", "at least one value is required");
    }
    let choice = args.choose(&mut rand::thread_rng()).unwrap();
    FnOutput::Text(choice.clone())
}
