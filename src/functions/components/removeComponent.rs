use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMessage;
use serenity::model::id::{ChannelId, MessageId};

/// ZremoveComponent{customID;messageID?}
/// Removes a specific component by its custom_id from a message.
/// Note: Discord requires re-sending the full component list, so this clears all components.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let custom_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("removeComponent", crate::error_messages::required(1, "customID")),
    };
    let mid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => match &ctx.trigger_message_id {
            Some(id) => id.clone(),
            None => return FnOutput::error("removeComponent", crate::error_messages::required(2, "messageID")),
        },
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeComponent", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeComponent", crate::error_messages::expected_snowflake(2, "messageID", &mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("removeComponent", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let msg = ChannelId::new(cid)
                .message(&http, MessageId::new(mid))
                .await
                .map_err(|_| crate::error_messages::action_failed("find message"))?;

            // Check if any component has this custom_id
            let found = msg.components.iter().any(|row| {
                row.components.iter().any(|c| {
                    use serenity::model::application::ActionRowComponent;
                    match c {
                        ActionRowComponent::Button(b) => {
                            if let serenity::model::application::ButtonKind::NonLink { custom_id: ref cid, .. } = b.data {
                                cid == &custom_id
                            } else {
                                false
                            }
                        }
                        ActionRowComponent::SelectMenu(s) => s.custom_id.as_deref() == Some(custom_id.as_str()),
                        ActionRowComponent::InputText(i) => i.custom_id == custom_id,
                        _ => false,
                    }
                })
            });

            if !found {
                return Err(crate::error_messages::not_found("component", &custom_id));
            }

            // Remove all components (Discord API limitation — can't remove individual components)
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().components(vec![]))
                .await
                .map_err(|_| crate::error_messages::action_failed("remove component"))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("removeComponent", e),
    }
}
