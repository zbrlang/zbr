use crate::context::{DiscordContext, FnOutput};
use serenity::builder::CreateStageInstance;
use serenity::model::id::ChannelId;

/// ZstageCreate{channelID;topic}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) { Some(s) if !s.is_empty() => s.clone(), _ => return FnOutput::error("stageCreate", crate::error_messages::required(1, "channelID")) };
    let topic = match args.get(1) { Some(s) if !s.is_empty() => s.clone(), _ => return FnOutput::error("stageCreate", crate::error_messages::required(2, "topic")) };
    let cid: u64 = match cid_str.parse() { Ok(id) => id, Err(_) => return FnOutput::error("stageCreate", crate::error_messages::expected_snowflake(1, "channel ID", &cid_str)) };
    let http = match &ctx.http { Some(h) => h.clone(), None => return FnOutput::error("stageCreate", crate::error_messages::action_failed("get HTTP client")) };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = CreateStageInstance::new(topic);
            ChannelId::new(cid).create_stage_instance(&http, builder).await.map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("stageCreate", e),
    }
}
