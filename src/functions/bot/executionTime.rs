use crate::context::{ DiscordContext, FnOutput };

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let elapsed = ctx.execution_start.elapsed();
    // Return elapsed time in milliseconds with 5 decimal places
    FnOutput::Text(format!("{:.5}ms", elapsed.as_secs_f64() * 1000.0))
}
