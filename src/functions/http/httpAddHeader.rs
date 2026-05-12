use crate::context::{DiscordContext, FnOutput};

/// ZhttpAddHeader{name;value}
/// Adds a header that will be sent with all subsequent HTTP requests in this execution.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpAddHeader", "header name is required"),
    };
    let value = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpAddHeader", "header value is required"),
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.http_headers.lock().await.insert(name, value);
        })
    });

    FnOutput::Empty
}
