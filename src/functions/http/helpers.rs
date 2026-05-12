use crate::context::DiscordContext;
use std::collections::HashMap;

/// Executes an HTTP request and stores the status + body in context.
/// Returns (status, body) or an error string.
pub fn do_request(
    method: &str,
    url: &str,
    body: Option<&str>,
    ctx: &DiscordContext,
) -> Result<(u16, String), String> {
    // Snapshot headers before entering block_in_place
    let headers: HashMap<String, String> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { ctx.http_headers.lock().await.clone() })
    });

    let method = method.to_uppercase();
    let url = url.to_string();
    let body = body.map(|s| s.to_string());

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();

            let mut req = match method.as_str() {
                "GET"    => client.get(&url),
                "POST"   => client.post(&url),
                "PUT"    => client.put(&url),
                "DELETE" => client.delete(&url),
                "PATCH"  => client.patch(&url),
                other    => return Err(format!("unsupported method: {}", other)),
            };

            for (k, v) in &headers {
                req = req.header(k.as_str(), v.as_str());
            }

            if let Some(b) = body {
                req = req.body(b);
            }

            let resp = req.send().await.map_err(|e| format!("request failed: {}", e))?;
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            Ok((status, text))
        })
    });

    match result {
        Ok((status, body)) => {
            // Store status and body in context for ZhttpStatus / ZhttpResult
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    *ctx.http_last_status.lock().await = status;
                    *ctx.http_last_body.lock().await = body.clone();
                })
            });
            Ok((status, body))
        }
        Err(e) => Err(e),
    }
}
