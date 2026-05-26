use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollAnswerVotes", "messageID is required"),
    };
    let answer_id_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollAnswerVotes", "answerID is required"),
    };
    let cid_str = args.get(2).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollAnswerVotes", format!("invalid channel ID: '{}'", cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollAnswerVotes", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("pollAnswerVotes", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).message(&http, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => match &msg.poll {
            Some(poll) => {
                let target_answer_id: serenity::model::id::AnswerId = match answer_id_str.parse() {
                    Ok(id) => id,
                    Err(_) => return FnOutput::error("pollAnswerVotes", format!("invalid answer ID: '{}'", answer_id_str)),
                };
                match &poll.results {
                    Some(results) => {
                        for ac in &results.answer_counts {
                            if ac.id == target_answer_id {
                                return FnOutput::Text(ac.count.to_string());
                            }
                        }
                        FnOutput::error("pollAnswerVotes", format!("answer ID '{}' not found", answer_id_str))
                    },
                    None => FnOutput::Text("0".to_string()),
                }
            },
            None => FnOutput::error("pollAnswerVotes", "message has no poll"),
        },
        Err(e) => FnOutput::error("pollAnswerVotes", e.to_string()),
    }
}
