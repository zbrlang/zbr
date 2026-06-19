use crate::context::{DiscordContext, FnOutput};
use rand::Rng;

/// Zdice{formula} — rolls dice (e.g., 1d6).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let formula = &args[0];
    let parts: Vec<&str> = formula.split('d').collect();
    if parts.len() != 2 {
        return FnOutput::error("dice", "invalid formula, use NdM (e.g., 1d6)".to_string());
    }
    let count: i32 = match parts[0].parse() { Ok(n) => n, Err(_) => return FnOutput::error("dice", "invalid count".to_string()) };
    let sides: i32 = match parts[1].parse() { Ok(n) => n, Err(_) => return FnOutput::error("dice", "invalid sides".to_string()) };
    
    if count <= 0 || sides <= 0 {
        return FnOutput::error("dice", "count and sides must be positive".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let total: i32 = (0..count).map(|_| rng.gen_range(1..=sides)).sum();
    FnOutput::Text(total.to_string())
}
