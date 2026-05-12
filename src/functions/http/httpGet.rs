use crate::context::{DiscordContext, FnOutput};
use super::helpers::do_request;

/// ZhttpGet{url}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpGet", "url is required"),
    };

    match do_request("GET", &url, None, ctx) {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("httpGet", e),
    }
}
