use crate::context::{DiscordContext, FnOutput};
use rand::Rng;

/// ZrandomCase{text}
/// Randomizes character casing.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = &args[0];
    let mut rng = rand::thread_rng();
    let result: String = text.chars().map(|c| {
        if rng.gen_bool(0.5) {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
    }).collect();

    FnOutput::Text(result)
}
