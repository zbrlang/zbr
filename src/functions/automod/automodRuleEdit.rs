use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditAutoModRule;
use serenity::model::guild::automod::{Action, EventType, KeywordPresetType, Trigger};
use serenity::model::id::{ChannelId, GuildId, RoleId, RuleId};

fn parse_trigger(json_str: &str) -> Result<Trigger, String> {
    let v: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| crate::error_messages::action_failed_reason("parse trigger JSON", &format!("{}", e)))?;

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
        _ => Err(crate::error_messages::expected_choice(5, "trigger type", "keyword, spam, keyword_preset, mention_spam", &t)),
    }
}

fn parse_actions(json_str: &str) -> Result<Vec<Action>, String> {
    let arr: Vec<serde_json::Value> =
        serde_json::from_str(json_str).map_err(|e| crate::error_messages::action_failed_reason("parse actions JSON", &format!("{}", e)))?;

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
                _ => Err(crate::error_messages::expected_choice(6, "action type", "block_message, alert, timeout", &t)),
            }
        })
        .collect()
}

/// ZautomodRuleEdit{guildID;ruleID;name?;eventType?;triggerJSON?;actionsJSON?;enabled?;exemptRoles?;exemptChannels?}
/// Edits an auto-moderation rule. Use !unchanged for optional fields to skip.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let rule_id_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("automodRuleEdit", crate::error_messages::expected_snowflake(1, "guild ID", &guild_id_str)),
    };

    let rule_id: u64 = match rule_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("automodRuleEdit", crate::error_messages::expected_snowflake(2, "rule ID", &rule_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("automodRuleEdit", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditAutoModRule::new();

            let name = args.get(2).filter(|s| !s.is_empty()).map(|s| s.as_str()).unwrap_or("!unchanged");
            if name != "!unchanged" {
                builder = builder.name(name);
            }

            let event_type_str = args
                .get(3)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "!unchanged".to_string());
            if event_type_str != "!unchanged" {
                match event_type_str.as_str() {
                    "messagesend" => builder = builder.event_type(EventType::MessageSend),
                    _ => {
                        return Err(crate::error_messages::expected_choice(
                            4,
                            "eventType",
                            "messagesend, !unchanged",
                            &event_type_str,
                        ))
                    }
                }
            }

            let trigger_json = args
                .get(4)
                .filter(|s| !s.is_empty())
                .map(|s| s.as_str())
                .unwrap_or("!unchanged");
            if trigger_json != "!unchanged" {
                let trigger = parse_trigger(trigger_json)?;
                builder = builder.trigger(trigger);
            }

            let actions_json = args
                .get(5)
                .filter(|s| !s.is_empty())
                .map(|s| s.as_str())
                .unwrap_or("!unchanged");
            if actions_json != "!unchanged" {
                let actions = parse_actions(actions_json)?;
                builder = builder.actions(actions);
            }

            let enabled_str = args
                .get(6)
                .filter(|s| !s.is_empty())
                .map(|s| s.as_str())
                .unwrap_or("!unchanged");
            if enabled_str != "!unchanged" {
                match enabled_str {
                    "true" => builder = builder.enabled(true),
                    "false" => builder = builder.enabled(false),
                    _ => return Err(crate::error_messages::expected_choice(
                        7,
                        "enabled",
                        "true, false, !unchanged",
                        enabled_str,
                    )),
                }
            }

            let exempt_roles_str = args.get(7).filter(|s| !s.is_empty()).map(|s| s.as_str()).unwrap_or("!unchanged");
            if exempt_roles_str != "!unchanged" {
                let roles: Vec<RoleId> = exempt_roles_str
                    .split(',')
                    .filter_map(|id| id.trim().parse::<u64>().ok().map(RoleId::new))
                    .collect();
                builder = builder.exempt_roles(roles);
            }

            let exempt_channels_str = args
                .get(8)
                .filter(|s| !s.is_empty())
                .map(|s| s.as_str())
                .unwrap_or("!unchanged");
            if exempt_channels_str != "!unchanged" {
                let channels: Vec<ChannelId> = exempt_channels_str
                    .split(',')
                    .filter_map(|id| id.trim().parse::<u64>().ok().map(ChannelId::new))
                    .collect();
                builder = builder.exempt_channels(channels);
            }

            GuildId::new(guild_id)
                .edit_automod_rule(&http, RuleId::new(rule_id), builder)
                .await
                .map(|r| r.id.to_string())
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(id) => FnOutput::Text(id),
        Err(e) => FnOutput::error("automodRuleEdit", e),
    }
}
