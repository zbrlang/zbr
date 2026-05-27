use crate::context::{DiscordContext, FnOutput};

// Zvar{name}        — get temp var
// Zvar{name;value}  — set temp var
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(n) => n.clone(),
        None => return FnOutput::error("var", crate::error_messages::required(1, "name")),
    };

    match args.get(1) {
        Some(value) => {
            let value = value.clone();
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.temp_vars.lock().await.insert(name, value);
                })
            });
            FnOutput::Empty
        }
        None => {
            let result = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.temp_vars.lock().await.get(&name).cloned().unwrap_or_default()
                })
            });
            FnOutput::Text(result)
        }
    }
}
