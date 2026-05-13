use crate::context::{DiscordContext, FnOutput};

/// ZhttpGetHeader{name}
/// Returns the value of a header previously set via ZhttpAddHeader{name;value}.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpGetHeader", "header name is required"),
    };

    let value = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { ctx.http_headers.lock().await.get(&name).cloned().unwrap_or_default() })
    });

    FnOutput::Text(value)
}

