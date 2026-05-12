use crate::context::{DiscordContext, FnOutput};
use super::helpers::do_request;

/// ZhttpDelete{url;body?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpDelete", "url is required"),
    };
    let body = args.get(1).filter(|s| !s.is_empty()).map(|s| s.as_str());

    match do_request("DELETE", &url, body, ctx) {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("httpDelete", e),
    }
}
