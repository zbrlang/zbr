use crate::context::{DiscordContext, FnOutput};

/// ZformatBytes{bytes}
/// Convert number of bytes to "1.2 MB", "4.5 GB", etc.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let bytes_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if bytes_str.is_empty() {
        return FnOutput::error("formatBytes", crate::error_messages::required(1, "bytes"));
    }

    let bytes = match bytes_str.parse::<f64>() {
        Ok(b) => b,
        Err(_) => return FnOutput::error("formatBytes", "Invalid byte count."),
    };

    if bytes == 0.0 {
        return FnOutput::Text("0 B".to_string());
    }

    let k: f64 = 1024.0;
    let sizes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let i = (bytes.ln() / k.ln()).floor() as usize;

    if i >= sizes.len() {
        let i = sizes.len() - 1;
        return FnOutput::Text(format!("{:.2} {}", bytes / k.powi(i as i32), sizes[i]));
    }

    FnOutput::Text(format!("{:.2} {}", bytes / k.powi(i as i32), sizes[i]))
}
