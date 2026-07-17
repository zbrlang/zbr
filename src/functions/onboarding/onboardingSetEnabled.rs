use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let enabled = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if enabled.is_empty() {
        return FnOutput::error("onboardingSetEnabled", crate::error_messages::required(1, "enabled"));
    }

    // Logic to set onboarding enabled status goes here.
    FnOutput::Empty
}
