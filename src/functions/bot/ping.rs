use crate::context::{ DiscordContext, FnOutput };

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let latency = ctx.shard_latency.map(|d| d.as_secs_f64() * 1000.0);
    match latency {
        Some(l) => FnOutput::Text(format!("{:.5}ms", l)),
        None => FnOutput::Text("N/A".to_string()),
    }
}
