use crate::context::{DiscordContext, FnOutput};
use rand::seq::SliceRandom;

/// ZlistShuffle{list;separator} — shuffles a list.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("listShuffle", "expected 2 arguments: list;separator".to_string());
    }
    let separator = &args[1];
    let mut items: Vec<&str> = args[0].split(separator).collect();
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
    FnOutput::Text(items.join(separator))
}
