use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let question = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollCreate", crate::error_messages::required(1, "question")),
    };
    let duration_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollCreate", crate::error_messages::required(2, "duration")),
    };
    let duration_h: i64 = match duration_str.parse() {
        Ok(d) => d,
        Err(_) => return FnOutput::error("pollCreate", crate::error_messages::expected_duration(2, "duration", &duration_str)),
    };
    let duration_s = duration_h * 3600;

    let temp = ctx.temp_vars.clone();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut vars = temp.lock().await;
            vars.insert("poll_question".to_string(), question);
            vars.insert("poll_answers".to_string(), "[]".to_string());
            vars.insert("poll_multiselect".to_string(), "false".to_string());
            vars.insert("poll_duration".to_string(), duration_s.to_string());
        })
    });

    FnOutput::Empty
}
