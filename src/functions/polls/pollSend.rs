use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{CreateMessage, CreatePoll, CreatePollAnswer};
use serenity::model::id::ChannelId;

fn parse_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None,
    }
}

/// ZpollSend{channelID?;returnID?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollSend", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let return_id = args.get(1).and_then(|s| parse_bool(s)).unwrap_or(false);

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("pollSend", "no HTTP client available"),
    };

    let temp = ctx.temp_vars.clone();
    let (question, answers_json, multiselect, duration_str) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let vars = temp.lock().await;
            let q = vars.get("poll_question").cloned().unwrap_or_default();
            let a = vars.get("poll_answers").cloned().unwrap_or_else(|| "[]".to_string());
            let m = vars.get("poll_multiselect").cloned().unwrap_or_else(|| "false".to_string());
            let d = vars.get("poll_duration").cloned().unwrap_or_default();
            (q, a, m, d)
        })
    });

    if question.is_empty() {
        return FnOutput::error("pollSend", crate::error_messages::requires_first("pollCreate"));
    }
    let duration_secs: u64 = match duration_str.parse() {
        Ok(d) => d,
        Err(_) => return FnOutput::error("pollSend", crate::error_messages::requires_first("pollCreate")),
    };

    let answers: Vec<serde_json::Value> = match serde_json::from_str(&answers_json) {
        Ok(v) => v,
        Err(_) => return FnOutput::error("pollSend", "invalid poll answers data"),
    };

    let poll_answers: Vec<CreatePollAnswer> = answers.iter().map(|a| {
        let text = a["text"].as_str().unwrap_or("");
        let emoji = a["emoji"].as_str().unwrap_or("");
        let mut builder = CreatePollAnswer::new().text(text);
        if !emoji.is_empty() {
            builder = builder.emoji(emoji.to_string());
        }
        builder
    }).collect();

    let duration = std::time::Duration::from_secs(duration_secs);
    let poll_builder = CreatePoll::new()
        .question(&question)
        .answers(poll_answers)
        .duration(duration);
    let poll_builder = if multiselect == "true" {
        poll_builder.allow_multiselect()
    } else {
        poll_builder
    };

    let message_builder = CreateMessage::new().poll(poll_builder);

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).send_message(&http, message_builder).await
        })
    });

    match result {
        Ok(msg) => {
            if return_id {
                FnOutput::Text(msg.id.to_string())
            } else {
                FnOutput::Empty
            }
        }
        Err(e) => FnOutput::error("pollSend", e.to_string()),
    }
}
