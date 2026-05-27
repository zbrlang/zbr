use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let text = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollAddAnswer", crate::error_messages::required(1, "text")),
    };
    let emoji = args.get(1).cloned().unwrap_or_default();

    let temp = ctx.temp_vars.clone();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut vars = temp.lock().await;
            let answers_json = vars.get("poll_answers").cloned().unwrap_or_else(|| "[]".to_string());
            let mut answers: Vec<serde_json::Value> = match serde_json::from_str(&answers_json) {
                Ok(v) => v,
                Err(_) => Vec::new(),
            };
            let entry = serde_json::json!({"text": text, "emoji": emoji});
            answers.push(entry);
            vars.insert("poll_answers".to_string(), serde_json::to_string(&answers).unwrap_or_else(|_| "[]".to_string()));
        })
    });

    FnOutput::Empty
}
