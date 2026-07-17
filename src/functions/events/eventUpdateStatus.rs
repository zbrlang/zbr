use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let eid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let status_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    if eid_str.is_empty() || status_str.is_empty() {
        return FnOutput::error("eventUpdateStatus", "Usage: ZeventUpdateStatus{eventID;status}");
    }

    // Logic to update event status using serenity goes here.
    FnOutput::Empty
}
