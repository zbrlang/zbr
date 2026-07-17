use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let poll_id = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let end_time = args.get(1).filter(|s| !s.is_empty()).cloned();

    if poll_id.is_empty() {
        return FnOutput::error("pollExpiration", "Usage: ZpollExpiration{pollID;endTime?}");
    }

    // Logic to get or set poll expiration goes here.
    FnOutput::Text("1721234567".to_string())
}
