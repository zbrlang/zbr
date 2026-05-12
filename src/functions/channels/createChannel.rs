use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, ChannelId};
use serenity::model::channel::ChannelType;
use serenity::builder::CreateChannel;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("createChannel", "name is required");
    }

    // type defaults to "text" when empty or omitted
    let type_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.to_lowercase(),
        _ => "text".to_string(),
    };
    // categoryID is optional; empty string means no category
    let cat_id_str = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => String::new(),
    };

    let channel_type = match type_str.as_str() {
        "text" => ChannelType::Text,
        "voice" => ChannelType::Voice,
        "category" => ChannelType::Category,
        "stage" => ChannelType::Stage,
        "forum" => ChannelType::Forum,
        _ => return FnOutput::error("createChannel", format!("invalid channel type: '{}' (expected text, voice, category, stage, or forum)", type_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createChannel", "not in a guild"),
    };

    let mut builder = CreateChannel::new(name).kind(channel_type);

    if !cat_id_str.is_empty() {
        let cat_id: u64 = match cat_id_str.parse() {
            Ok(id) => id,
            Err(_) => return FnOutput::error("createChannel", format!("invalid category ID: '{}'", cat_id_str)),
        };
        builder = builder.category(ChannelId::new(cat_id));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createChannel", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).create_channel(&http, builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("createChannel", format!("failed to create channel: {}", e)),
    }
}
