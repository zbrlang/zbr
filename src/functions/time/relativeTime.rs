use crate::context::{DiscordContext, FnOutput};
use chrono::{Utc};

/// ZrelativeTime{timestamp}
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let ts: i64 = match args.get(0).and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return FnOutput::error("relativeTime", crate::error_messages::required(1, "timestamp")),
    };
    
    let now = Utc::now().timestamp();
    let diff = now - ts;
    
    let result = if diff < 0 {
        "in the future".to_string()
    } else if diff < 60 {
        format!("{} seconds ago", diff)
    } else if diff < 3600 {
        format!("{} minutes ago", diff / 60)
    } else if diff < 86400 {
        format!("{} hours ago", diff / 3600)
    } else {
        format!("{} days ago", diff / 86400)
    };
    
    FnOutput::Text(result)
}
