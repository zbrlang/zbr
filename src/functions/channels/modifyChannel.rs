use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;
use serenity::builder::EditChannel;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_default();
    if cid_str.is_empty() {
        return FnOutput::error("modifyChannel", "channel ID is required");
    }

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("modifyChannel", format!("invalid channel ID: '{}'", cid_str)),
    };

    let name = args.get(1).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let topic = args.get(2).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let nsfw_str = args.get(3).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let position_str = args.get(4).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let category_id_str = args.get(5).cloned().unwrap_or_else(|| "!unchanged".to_string());

    let mut builder = EditChannel::new();

    if name != "!unchanged" {
        builder = builder.name(name);
    }
    if topic != "!unchanged" {
        builder = builder.topic(topic);
    }
    if nsfw_str != "!unchanged" {
        let nsfw = match nsfw_str.as_str() {
            "true" => true,
            "false" => false,
            _ => return FnOutput::error("modifyChannel", format!("invalid boolean for nsfw: '{}'", nsfw_str)),
        };
        builder = builder.nsfw(nsfw);
    }
    if position_str != "!unchanged" {
        let pos: u16 = match position_str.parse() {
            Ok(p) => p,
            Err(_) => return FnOutput::error("modifyChannel", format!("invalid position: '{}'", position_str)),
        };
        builder = builder.position(pos);
    }
    if category_id_str != "!unchanged" {
        if category_id_str.is_empty() {
            // How to remove category? Set parent_id to None... wait serenity EditChannel might not support removing parent easily?
            // Passing None to category() clears it.
            builder = builder.category(None);
        } else {
            let cat_id: u64 = match category_id_str.parse() {
                Ok(id) => id,
                Err(_) => return FnOutput::error("modifyChannel", format!("invalid category ID: '{}'", category_id_str)),
            };
            builder = builder.category(Some(ChannelId::new(cat_id)));
        }
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("modifyChannel", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid).edit(&http, builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("modifyChannel", "channel not found"),
    }
}
