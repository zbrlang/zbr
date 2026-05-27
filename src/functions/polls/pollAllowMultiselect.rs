use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let value = match args.get(0) {
        Some(s) if s == "true" || s == "false" => s.clone(),
        _ => return FnOutput::error("pollAllowMultiselect", crate::error_messages::expected_boolean(1, "value", &args[0])),
    };

    let temp = ctx.temp_vars.clone();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut vars = temp.lock().await;
            vars.insert("poll_multiselect".to_string(), value);
        })
    });

    FnOutput::Empty
}
