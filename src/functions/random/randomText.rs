use crate::context::{DiscordContext, FnOutput};
use rand::seq::SliceRandom;

/// ZrandomText{text1;text2;...} — picks one value randomly.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("randomText", crate::error_messages::too_few_args(1, args.len()));
    }
    let choice = args.choose(&mut rand::thread_rng()).unwrap();
    FnOutput::Text(choice.clone())
}
