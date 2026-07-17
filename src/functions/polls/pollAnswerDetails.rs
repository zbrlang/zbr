use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let poll_id = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let answer_id = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    if poll_id.is_empty() || answer_id.is_empty() {
        return FnOutput::error("pollAnswerDetails", "Usage: ZpollAnswerDetails{pollID;answerID}");
    }

    // Logic to return JSON with answer details goes here.
    FnOutput::Text("{\"text\": \"Option A\", \"count\": 10}".to_string())
}
