use crate::context::{DiscordContext, FnOutput};
use std::collections::HashMap;

/// Zentropy{text}
/// Calculate Shannon entropy.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if text.is_empty() {
        return FnOutput::Text("0".to_string());
    }

    let mut counts = HashMap::new();
    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let len = text.chars().count() as f64;
    let mut entropy = 0.0;

    for &count in counts.values() {
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }

    FnOutput::Text(format!("{:.4}", entropy))
}
