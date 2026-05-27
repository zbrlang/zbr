use crate::context::{DiscordContext, FnOutput};

/// ZhttpRemoveHeader{name}
/// Removes a header previously set via ZhttpAddHeader{name;value}.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("httpRemoveHeader", crate::error_messages::required(1, "header name")),
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.http_headers.lock().await.remove(&name);
        })
    });

    FnOutput::Empty
}

