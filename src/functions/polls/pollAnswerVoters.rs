use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, MessageId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollAnswerVoters", crate::error_messages::required(1, "messageID")),
    };
    let answer_id_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("pollAnswerVoters", crate::error_messages::required(2, "answerID")),
    };
    let cid_str = args.get(2).cloned().unwrap_or_else(|| ctx.channel_id.clone());
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollAnswerVoters", crate::error_messages::expected_snowflake(3, "channelID", &cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollAnswerVoters", crate::error_messages::expected_snowflake(1, "messageID", &mid_str)),
    };
    let answer_id: serenity::model::id::AnswerId = match answer_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pollAnswerVoters", crate::error_messages::expected_snowflake(2, "answerID", &answer_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("pollAnswerVoters", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .get_poll_answer_voters(
                    &http,
                    MessageId::new(mid),
                    answer_id,
                    None,
                    None,
                )
                .await
        })
    });

    match result {
        Ok(users) => {
            let user_ids: Vec<String> = users.iter().map(|u| u.id.to_string()).collect();
            FnOutput::Text(serde_json::to_string(&user_ids).unwrap_or_else(|_| "[]".to_string()))
        },
        Err(e) => FnOutput::error("pollAnswerVoters", e.to_string()),
    }
}
