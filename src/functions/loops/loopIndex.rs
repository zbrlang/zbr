use crate::context::{DiscordContext, FnOutput};

/// ZloopIndex{} — returns the current 1-based loop iteration index.
/// Returns "0" if called outside a loop.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let val = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.temp_vars.lock().await
                .get("__loop_index")
                .cloned()
                .unwrap_or_else(|| "0".to_string())
        })
    });
    FnOutput::Text(val)
}
