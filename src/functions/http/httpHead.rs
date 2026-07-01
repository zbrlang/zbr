use crate::context::{DiscordContext, FnOutput};
use super::helpers::do_request;

/// ZhttpHead{url}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpHead", crate::error_messages::required(1, "url")),
    };

    match do_request("HEAD", &url, None, ctx) {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("httpHead", e),
    }
}
