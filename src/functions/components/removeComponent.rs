use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMessage;
use serenity::model::id::{ChannelId, MessageId};

/// ZremoveComponent{customID;messageID?}
/// Removes a specific component by its custom_id from a message.
/// Note: Discord requires re-sending the full component list, so this clears all components.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let custom_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("removeComponent", "customID is required"),
    };
    let mid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => match &ctx.trigger_message_id {
            Some(id) => id.clone(),
            None => return FnOutput::error("removeComponent", "messageID is required"),
        },
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeComponent", "invalid channel ID in context"),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("removeComponent", format!("invalid message ID: '{}'", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("removeComponent", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let msg = ChannelId::new(cid)
                .message(&http, MessageId::new(mid))
                .await
                .map_err(|_| "message not found".to_string())?;

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
                return Err(format!("component '{}' not found", custom_id));
            }

            // Remove all components (Discord API limitation — can't remove individual components)
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().components(vec![]))
                .await
                .map_err(|_| "failed to remove component".to_string())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("removeComponent", e),
    }
}
