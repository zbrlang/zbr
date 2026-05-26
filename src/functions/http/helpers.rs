use crate::context::DiscordContext;
use std::collections::HashMap;
use std::net::{IpAddr, ToSocketAddrs};
use url::Url;

fn is_private_or_reserved(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
                || v4.is_documentation()
        }
        IpAddr::V6(v6) => {
            v6.is_loopback() || v6.is_unspecified() || v6.is_unique_local()
        }
    }
}

/// Validates a URL against SSRF blocklists — rejects requests to
/// private/reserved IPs and known dangerous hostnames.
fn validate_url(url_str: &str) -> Result<(), String> {
    let parsed = Url::parse(url_str).map_err(|_| "invalid URL".to_string())?;

    let host = parsed.host().ok_or("URL has no host")?;

    match host {
        url::Host::Domain(domain) => {
            let lower = domain.to_lowercase();
            let dangerous: &[&str] = &[
                "localhost",
                "localhost.localdomain",
                "127.0.0.1",
                "::1",
                "0.0.0.0",
                "metadata",
                "metadata.google.internal",
                "metadata.google.internal.",
            ];
            if dangerous.contains(&lower.as_str()) {
                return Err("requests to this host are not allowed".to_string());
            }

            // Resolve hostname and check whether it points to a private/reserved IP
            if let Ok(addrs) = format!("{}:0", lower).to_socket_addrs() {
                for addr in addrs {
                    if is_private_or_reserved(&addr.ip()) {
                        return Err(
                            "requests to private or reserved IPs are not allowed".to_string(),
                        );
                    }
                }
            }
        }
        url::Host::Ipv4(ip) => {
            if is_private_or_reserved(&IpAddr::V4(ip)) {
                return Err("requests to private or reserved IPs are not allowed".to_string());
            }
        }
        url::Host::Ipv6(ip) => {
            if is_private_or_reserved(&IpAddr::V6(ip)) {
                return Err("requests to private or reserved IPs are not allowed".to_string());
            }
        }
    }

    Ok(())
}

/// Executes an HTTP request and stores the status + body in context.
/// Returns (status, body) or an error string.
pub fn do_request(
    method: &str,
    url: &str,
    body: Option<&str>,
    ctx: &DiscordContext,
) -> Result<(u16, String), String> {
    // Validate URL against SSRF blocklist
    validate_url(url)?;

    // Snapshot headers before entering block_in_place
    let mut headers: HashMap<String, String> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { ctx.http_headers.lock().await.clone() })
    });

    let method = method.to_uppercase();
    let url = url.to_string();
    let body = body.map(|s| s.to_string());

    // Auto-detect Content-Type for JSON-like bodies
    let has_content_type = headers.keys().any(|k| k.to_lowercase() == "content-type");
    if !has_content_type {
        if let Some(ref b) = body {
            let trimmed = b.trim_start();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                headers.insert("Content-Type".to_string(), "application/json".to_string());
            }
        }
    }

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
