use crate::context::{DiscordContext, FnOutput};

/// ZhttpStatus{}
/// Returns the HTTP status code of the last HTTP request made in this execution.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let status = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { *ctx.http_last_status.lock().await })
    });
    FnOutput::Text(status.to_string())
}
