use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::channel::ChannelType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cat_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if cat_id_str.is_empty() {
        return FnOutput::error("categoryChannels", crate::error_messages::required(1, "category ID"));
    }

    let cat_id: u64 = match cat_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("categoryChannels", crate::error_messages::expected_snowflake(1, "category ID", &cat_id_str)),
    };

    let separator = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "\n".to_string());
    let return_type = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "name".to_string()).to_lowercase();

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("categoryChannels", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("categoryChannels", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channels = GuildId::new(gid).channels(&http).await.map_err(|e| e.to_string())?;
            Ok::<_, String>(channels)
        })
    });

    match result {
        Ok(channels) => {
            let mut is_category = false;
            for (id, c) in &channels {
                if id.get() == cat_id && c.kind == ChannelType::Category {
                    is_category = true;
                    break;
                }
            }

            if !is_category {
                return FnOutput::error("categoryChannels", crate::error_messages::not_found("category", &cat_id_str));
            }

            let mut filtered = Vec::new();
            for c in channels.values() {
                if let Some(parent) = c.parent_id {
                    if parent.get() == cat_id {
                        filtered.push(c);
                    }
                }
            }

            // Sort by position
            filtered.sort_by_key(|c| c.position);

            if return_type == "count" {
                return FnOutput::Text(filtered.len().to_string());
            }

            let mut list = Vec::new();
            for c in filtered {
                match return_type.as_str() {
                    "id" => list.push(c.id.to_string()),
                    "mention" => list.push(format!("<#{}>", c.id)),
                    _ => list.push(c.name.clone()),
                }
            }
            FnOutput::Text(list.join(&separator))
        }
        Err(e) => FnOutput::error("categoryChannels", crate::error_messages::action_failed_reason("fetch channels", &e)),
    }
}
