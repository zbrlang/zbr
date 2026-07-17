use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mode = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if mode.is_empty() {
        return FnOutput::error("onboardingSetMode", crate::error_messages::required(1, "mode"));
    }

    // Logic to set onboarding mode goes here.
    FnOutput::Empty
}
