use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_index, validate_bool, validate_embed_sendable, validate_url};
use serde_json::{json, Value};

/// ZsendWebhook{webhookURL;(index);(returnID)}
/// Sends the embed at the given index via a webhook URL.
/// Uses the Discord webhook API directly — no bot token required.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = args.get(0).cloned().unwrap_or_default();
    if let Err(e) = validate_url(&url, "sendWebhook") { return e; }

    let index = match parse_index(args.get(1), "sendWebhook") {
        Ok(i) => i, Err(e) => return e,
    };

    let return_id = match args.get(2) {
        Some(s) => match validate_bool(s, "sendWebhook") {
            Ok(b) => b, Err(e) => return e,
        },
        None => false,
    };

    let embed_data = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.embed.lock().await.get(index).cloned()
        })
    });

    let embed_data = match embed_data {
        Some(e) if e.has_content() => e,
        Some(_) => return FnOutput::error("sendWebhook", crate::error_messages::action_failed_reason("send embed", &format!("embed {} has no content", index + 1))),
        None    => return FnOutput::error("sendWebhook", crate::error_messages::not_found("embed", &(index + 1).to_string())),
    };

    if let Err(e) = validate_embed_sendable(&embed_data, "sendWebhook", index) { return e; }

    // Build the embed JSON manually for the webhook payload
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut embed_json = serde_json::Map::new();

            if let Some(t) = &embed_data.title { embed_json.insert("title".into(), json!(t)); }
            if let Some(u) = &embed_data.title_url { embed_json.insert("url".into(), json!(u)); }
            if let Some(d) = &embed_data.description { embed_json.insert("description".into(), json!(d)); }
            if let Some(c) = embed_data.color { embed_json.insert("color".into(), json!(c)); }
            if let Some(t) = &embed_data.thumbnail { embed_json.insert("thumbnail".into(), json!({"url": t})); }
            if let Some(i) = &embed_data.image { embed_json.insert("image".into(), json!({"url": i})); }

            if let Some(f) = &embed_data.footer {
                let mut footer = json!({"text": f});
                if let Some(fi) = &embed_data.footer_icon {
                    footer["icon_url"] = json!(fi);
                }
                embed_json.insert("footer".into(), footer);
            }

            if let Some(a) = &embed_data.author {
                let mut author = json!({"name": a});
                if let Some(ai) = &embed_data.author_icon { author["icon_url"] = json!(ai); }
                if let Some(au) = &embed_data.author_url { author["url"] = json!(au); }
                embed_json.insert("author".into(), author);
            }

            if embed_data.timestamp {
                embed_json.insert("timestamp".into(), json!(chrono::Utc::now().to_rfc3339()));
            }

            if !embed_data.fields.is_empty() {
                let fields: Vec<Value> = embed_data.fields.iter().map(|f| {
                    json!({"name": f.name, "value": f.value, "inline": f.inline})
                }).collect();
                embed_json.insert("embeds".into(), json!(fields));
            }

            let payload = json!({ "embeds": [Value::Object(embed_json)] });

            let post_url = if return_id {
                format!("{}?wait=true", url)
            } else {
                url.clone()
            };

            let client = reqwest::Client::new();
            let resp = client.post(&post_url)
                .json(&payload)
                .send().await
                .map_err(|e| format!("failed to send webhook: {}", e))?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(format!("webhook returned {}: {}", status, body));
            }

            if return_id {
                let msg: Value = resp.json().await
                    .map_err(|e| format!("failed to parse webhook response: {}", e))?;
                Ok(msg["id"].as_str().unwrap_or("").to_string())
            } else {
                Ok(String::new())
            }
        })
    });

    match result {
        Err(e) => FnOutput::Error(e),
        Ok(id) => {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.consumed_embeds.lock().await.insert(index);
                })
            });
            if id.is_empty() { FnOutput::Empty } else { FnOutput::Text(id) }
        }
    }
}
