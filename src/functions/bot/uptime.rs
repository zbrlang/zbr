use crate::context::{DiscordContext, FnOutput, START_TIME};

pub fn run(_args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let start = match START_TIME.get() {
        Some(s) => s,
        None => return FnOutput::Text("0s".to_string()),
    };

    let duration = start.elapsed();
    let total_secs = duration.as_secs();
    
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
        parts.push(format!("{}h", hours));
        parts.push(format!("{}m", mins));
        parts.push(format!("{}s", secs));
    } else if hours > 0 {
        parts.push(format!("{}h", hours));
        parts.push(format!("{}m", mins));
        parts.push(format!("{}s", secs));
    } else if mins > 0 {
        parts.push(format!("{}m", mins));
        parts.push(format!("{}s", secs));
    } else {
        parts.push(format!("{}s", secs));
    }
    
    FnOutput::Text(parts.join(" "))
}
