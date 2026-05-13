use crate::context::{DiscordContext, FnOutput};

/// ZloopValue{} — returns the current loop element value.
/// Returns "" if called outside a loop.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let val = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.temp_vars.lock().await
                .get("__loop_value")
                .cloned()
                .unwrap_or_default()
        })
    });
    FnOutput::Text(val)
}
