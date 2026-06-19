use crate::context::{DiscordContext, FnOutput};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// ZweightedRandom{items;weights} — returns an item based on weights.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("weightedRandom", "expected 2 arguments: items;weights".to_string());
    }
    let items: Vec<&str> = args[0].split(',').collect();
    let weights: Vec<u32> = match args[1].split(',').map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, _>>() {
        Ok(w) => w,
        Err(_) => return FnOutput::error("weightedRandom", "invalid weights".to_string()),
    };
    if items.len() != weights.len() {
        return FnOutput::error("weightedRandom", "items and weights count mismatch".to_string());
    }
    let dist = match WeightedIndex::new(&weights) {
        Ok(d) => d,
        Err(_) => return FnOutput::error("weightedRandom", "invalid weights distribution".to_string()),
    };
    let mut rng = rand::thread_rng();
    FnOutput::Text(items[dist.sample(&mut rng)].to_string())
}
