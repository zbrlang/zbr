use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditAutoModRule;
use serenity::model::guild::automod::{Action, EventType, KeywordPresetType, Trigger};
use serenity::model::id::{ChannelId, GuildId, RoleId};

fn parse_trigger(json_str: &str) -> Result<Trigger, String> {
    let v: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| format!("invalid trigger JSON: {}", e))?;

    let t = v
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_lowercase();

    match t.as_str() {
        "keyword" => {
            let strings = v
                .get("keyword_filter")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let regex_patterns = v
                .get("regex_patterns")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let allow_list = v
                .get("allow_list")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            Ok(Trigger::Keyword {
                strings,
                regex_patterns,
                allow_list,
            })
        }
        "spam" => Ok(Trigger::Spam),
        "keyword_preset" => {
            let presets = v
                .get("presets")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str())
                        .filter_map(|s| match s.to_lowercase().as_str() {
                            "profanity" => Some(KeywordPresetType::Profanity),
                            "sexual_content" => Some(KeywordPresetType::SexualContent),
                            "slurs" => Some(KeywordPresetType::Slurs),
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            let allow_list = v
                .get("allow_list")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            Ok(Trigger::KeywordPreset {
                presets,
                allow_list,
            })
        }
        "mention_spam" => {
            let limit = v
                .get("mention_total_limit")
                .and_then(|v| v.as_u64())
                .unwrap_or(5) as u8;
            Ok(Trigger::MentionSpam {
                mention_total_limit: limit,
            })
        }
        _ => Err(format!("unknown trigger type: '{}' (expected keyword, spam, keyword_preset, or mention_spam)", t)),
    }
}

fn parse_actions(json_str: &str) -> Result<Vec<Action>, String> {
    let arr: Vec<serde_json::Value> =
        serde_json::from_str(json_str).map_err(|e| format!("invalid actions JSON: {}", e))?;

    arr.iter()
        .map(|v| {
            let t = v
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_lowercase();
            match t.as_str() {
                "block_message" => Ok(Action::BlockMessage {
                    custom_message: v
                        .get("custom_message")
                        .and_then(|v| v.as_str().map(String::from)),
                }),
                "alert" => {
                    let cid_str = v
                        .get("channel_id")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| "alert action requires channel_id".to_string())?;
                    let cid: u64 = cid_str
                        .parse()
                        .map_err(|_| format!("invalid channel_id: '{}'", cid_str))?;
                    Ok(Action::Alert(ChannelId::new(cid)))
                }
                "timeout" => {
                    let secs = v
                        .get("duration_seconds")
                        .and_then(|v| v.as_u64())
                        .ok_or_else(|| "timeout action requires duration_seconds".to_string())?;
                    Ok(Action::Timeout(std::time::Duration::from_secs(secs)))
                }
                _ => Err(format!("unknown action type: '{}' (expected block_message, alert, or timeout)", t)),
            }
        })
        .collect()
}

/// ZautomodRuleCreate{guildID;name;eventType;triggerJSON;actionsJSON;enabled?;exemptRoles?;exemptChannels?}
/// Creates an auto-moderation rule. eventType: "messagesend". triggerJSON/actionsJSON: see docs.
/// exemptRoles/exemptChannels: comma-separated IDs.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).cloned().unwrap_or_default();
    let name = args.get(1).cloned().unwrap_or_default();
    let event_type_str = args
        .get(2)
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let trigger_json = args.get(3).cloned().unwrap_or_default();
    let actions_json = args.get(4).cloned().unwrap_or_default();

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("automodRuleCreate", "invalid guild ID"),
    };

    if name.is_empty() {
        return FnOutput::error("automodRuleCreate", "name is required");
    }

    let event_type = match event_type_str.as_str() {
        "messagesend" => EventType::MessageSend,
        _ => {
            return FnOutput::error(
                "automodRuleCreate",
                "eventType must be 'messagesend'",
            )
        }
    };

    let trigger: Trigger = match parse_trigger(&trigger_json) {
        Ok(t) => t,
        Err(e) => return FnOutput::error("automodRuleCreate", e),
    };

    let actions: Vec<Action> = match parse_actions(&actions_json) {
        Ok(a) => a,
        Err(e) => return FnOutput::error("automodRuleCreate", e),
    };

    let enabled = match args.get(5).map(|s| s.as_str()).unwrap_or("true") {
        "true" => true,
        "false" => false,
        _ => return FnOutput::error("automodRuleCreate", "enabled must be true or false"),
    };

    let exempt_roles = args
        .get(6)
        .map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<u64>().ok().map(RoleId::new))
                .collect::<Vec<RoleId>>()
        })
        .unwrap_or_default();

    let exempt_channels = args
        .get(7)
        .map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<u64>().ok().map(ChannelId::new))
                .collect::<Vec<ChannelId>>()
        })
        .unwrap_or_default();

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("automodRuleCreate", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditAutoModRule::new()
                .name(&name)
                .trigger(trigger)
                .actions(actions)
                .event_type(event_type)
                .enabled(enabled);

            let builder = if !exempt_roles.is_empty() {
                builder.exempt_roles(exempt_roles)
            } else {
                builder
            };

            let builder = if !exempt_channels.is_empty() {
                builder.exempt_channels(exempt_channels)
            } else {
                builder
            };

            GuildId::new(guild_id)
                .create_automod_rule(&http, builder)
                .await
                .map(|r| r.id.to_string())
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("automodRuleCreate", e),
    }
}
