use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollGet", crate::error_messages::required(1, "messageID")),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollGet", crate::error_messages::expected_snowflake(1, "messageID", &mid_str)),
    };
    let cid_str = args.get(1).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollGet", crate::error_messages::expected_snowflake(2, "channelID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("pollGet", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => match &msg.poll {
            Some(poll) => {
                let json = serde_json::json!({
                    "question": poll.question.text,
                    "answers": poll.answers.iter().map(|a| {
                        serde_json::json!({
                            "answer_id": a.answer_id.get(),
                            "text": a.poll_media.text,
                            "emoji": a.poll_media.emoji.as_ref().map(|e| format!("{:?}", e)),
                        })
                    }).collect::<Vec<_>>(),
                    "allow_multiselect": poll.allow_multiselect,
                    "layout_type": format!("{:?}", poll.layout_type),
                    "results": poll.results.as_ref().map(|r| {
                        serde_json::json!({
                            "is_finalized": r.is_finalized,
                            "answer_counts": r.answer_counts.iter().map(|ac| {
                                serde_json::json!({
                                    "id": ac.id.get(),
                                    "count": ac.count,
                                    "me_voted": ac.me_voted,
                                })
                            }).collect::<Vec<_>>(),
                        })
                    }),
                });
                FnOutput::Text(serde_json::to_string_pretty(&json).unwrap_or_else(|_| "{}".to_string()))
            },
            None => FnOutput::error("pollGet", "message has no poll"),
        },
        Err(e) => FnOutput::error("pollGet", e.to_string()),
    }
}
