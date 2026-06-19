use crate::context::{DiscordContext, FnOutput};
use rand::Rng;

/// ZcolorRandom{} — returns a random hex color.
pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mut rng = rand::thread_rng();
    let color: u32 = rng.gen_range(0..=0xFFFFFF);
    FnOutput::Text(format!("#{:06X}", color))
}
