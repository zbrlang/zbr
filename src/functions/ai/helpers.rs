use serde_json::json;

/// List all available Gemini models for debugging
pub fn list_gemini_models(api_key: &str) -> Result<String, String> {
    let url = format!("https://generativelanguage.googleapis.com/v1/models?key={}", api_key);

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let resp = client
                .get(&url)
                .send().await
                .map_err(|e| format!("request failed: {}", e))?;

            let status = resp.status().as_u16();
            let text = resp.text().await.map_err(|e| format!("failed to read response: {}", e))?;

            if status != 200 {
                return Err(format!("API error ({}): {}", status, text));
            }

            Ok(text)
        })
    })?;

    // Pretty print the model list
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&result) {
        if let Some(models) = v["models"].as_array() {
            let mut model_names = vec![];
            for model in models {
                if let Some(name) = model["name"].as_str() {
                    // Check if it supports generateContent
                    let supports_generate = model["supportedGenerationMethods"]
                        .as_array()
                        .map(|methods|
                            methods.iter().any(|m| m.as_str() == Some("generateContent"))
                        )
                        .unwrap_or(false);

                    if supports_generate {
                        model_names.push(name.to_string());
                    }
                }
            }
            return Ok(
                format!(
                    "Available models that support generateContent:\n{}",
                    model_names.join("\n")
                )
            );
        }
    }

    Ok(result)
}

/// Detected AI provider based on API key prefix
pub enum AiProvider {
    Gemini,
    // Future: OpenAI, Groq, etc.
}

/// Detect provider from API key signature
pub fn detect_provider(api_key: &str) -> Result<AiProvider, String> {
    if api_key.starts_with("AIza") || api_key.starts_with("AQ") {
        Ok(AiProvider::Gemini)
    } else {
        Err(
            format!(
                "unrecognized API key format — currently only Gemini (AIza... or AQ...) is supported"
            )
        )
    }
}

/// Send a chat request to Gemini and return the response text
pub fn gemini_chat(
    api_key: &str,
    system_prompt: Option<&str>,
    user_prompt: &str,
    model: &str,
    max_tokens: Option<u32>,
    temperature: Option<f64>
) -> Result<String, String> {
    let model = if model.is_empty() { "gemini-2.5-flash-lite" } else { model };
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key
    );

    // Build contents array
    let mut contents = vec![];

    // Gemini uses a system_instruction field separate from contents
    let system_instruction = system_prompt
        .filter(|s| !s.is_empty())
        .map(|s| { json!({
            "parts": [{ "text": s }]
        }) });

    contents.push(json!({
        "role": "user",
        "parts": [{ "text": user_prompt }]
    }));

    let mut body = json!({
        "contents": contents
    });

    if let Some(si) = system_instruction {
        body["systemInstruction"] = si;
    }

    // Generation config
    let mut gen_config = serde_json::Map::new();
    if let Some(t) = temperature {
        gen_config.insert("temperature".to_string(), json!(t));
    }
    if let Some(mt) = max_tokens {
        gen_config.insert("maxOutputTokens".to_string(), json!(mt));
    }
    if !gen_config.is_empty() {
        body["generationConfig"] = serde_json::Value::Object(gen_config);
    }

    let body_str = body.to_string();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let resp = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body_str)
                .send().await
                .map_err(|e| format!("request failed: {}", e))?;

            let status = resp.status().as_u16();

            let text = resp.text().await.map_err(|e| format!("failed to read response: {}", e))?;

            if status != 200 {
                // Try to extract error message from Gemini error response
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(msg) = v["error"]["message"].as_str() {
                        return Err(format!("Gemini API error ({}): {}", status, msg));
                    }
                }
                // If we can't parse the error, return the full response for debugging
                return Err(format!("Gemini API error ({}): {}", status, text));
            }

            Ok(text)
        })
    })?;

    // Parse Gemini response: candidates[0].content.parts[0].text
    let v: serde_json::Value = serde_json
        ::from_str(&result)
        .map_err(|_| "failed to parse Gemini response".to_string())?;

    v["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "no text in Gemini response".to_string())
}

/// Send an image + prompt to Gemini Vision and return the response text
pub fn gemini_vision(
    api_key: &str,
    image_url: &str,
    prompt: &str,
    model: &str
) -> Result<String, String> {
    // Download image bytes first
    let image_data = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let client = reqwest::Client::new();
            let resp = client
                .get(image_url)
                .send().await
                .map_err(|e| format!("failed to fetch image: {}", e))?;

            let content_type = resp
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("image/jpeg")
                .to_string();

            // Strip parameters like "; charset=utf-8"
            let mime = content_type.split(';').next().unwrap_or("image/jpeg").trim().to_string();

            let bytes = resp.bytes().await.map_err(|e| format!("failed to read image: {}", e))?;

            Ok::<(String, Vec<u8>), String>((mime, bytes.to_vec()))
        })
    })?;

    let (mime_type, bytes) = image_data;
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);

    let model = if model.is_empty() { "gemini-2.5-flash-lite" } else { model };
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key
    );

    let body =
        json!({
        "contents": [{
            "parts": [
                {
                    "inline_data": {
                        "mime_type": mime_type,
                        "data": b64
                    }
                },
                { "text": prompt }
            ]
        }]
    });

    let body_str = body.to_string();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let resp = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body_str)
                .send().await
                .map_err(|e| format!("request failed: {}", e))?;

            let status = resp.status().as_u16();
            let text = resp.text().await.map_err(|e| format!("failed to read response: {}", e))?;

            if status != 200 {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(msg) = v["error"]["message"].as_str() {
                        return Err(format!("Gemini API error ({}): {}", status, msg));
                    }
                }
                // If we can't parse the error, return the full response for debugging
                return Err(format!("Gemini API error ({}): {}", status, text));
            }

            Ok(text)
        })
    })?;

    let v: serde_json::Value = serde_json
        ::from_str(&result)
        .map_err(|_| "failed to parse Gemini response".to_string())?;

    v["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "no text in Gemini response".to_string())
}
