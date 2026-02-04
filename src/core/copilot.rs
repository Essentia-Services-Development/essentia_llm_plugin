//! External Code Assist API Client
//!
//! Provides integration with external code assistance APIs for enhanced
//! code completion and AI-powered development workflows.

#[allow(dead_code)]
#[derive(Clone)]
pub struct ExternalCodeAssistModels;

#[allow(dead_code)]
impl ExternalCodeAssistModels {
    pub fn get_available_models() -> Vec<&'static str> {
        vec!["ext-api-pro", "ext-api-standard", "ext-api-turbo", "ext-api-latest"]
    }
}

#[allow(dead_code)]
pub struct ExternalCodeAssist {
    model: String,
}

#[allow(dead_code)]
impl ExternalCodeAssist {
    pub fn new(model: &str) -> Self {
        Self { model: model.to_string() }
    }

    pub fn chat_with_api(
        &self, api_token: &str, message: &str, context: &[String],
    ) -> Result<String, String> {
        // External Code Assist API endpoint for completions
        let url = "https://api.essentia.ai/code_assist/v2/completions";

        // Build JSON body manually
        let mut body = format!(r#"{{"model":"{}","messages":["#, self.model);

        // Add context messages
        for (i, ctx) in context.iter().enumerate() {
            if i > 0 {
                body.push(',');
            }
            body.push_str(&format!(
                r#"{{"role":"system","content":"{}"}}"#,
                escape_json(ctx)
            ));
        }

        // Add user message
        if !context.is_empty() {
            body.push(',');
        }
        body.push_str(&format!(
            r#"{{"role":"user","content":"{}"}}"#,
            escape_json(message)
        ));
        body.push_str(r#"],"stream":false,"temperature":0.7}"#);

        // Use HTTPS client with auth
        let response =
            crate::essentia::http::post_with_auth(url, &format!("Bearer {}", api_token), &body)
                .map_err(|e| format!("HTTP request failed: {}", e))?;

        // Parse the response body as JSON
        let response_text = std::str::from_utf8(&response.body)
            .map_err(|_| "Invalid UTF-8 in response".to_string())?;

        let json = crate::essentia::json::parse(response_text)
            .map_err(|_| "Failed to parse JSON response".to_string())?;

        // Extract the completion
        if let Some(choices) = json.get("choices")
            && let Some(choice) = choices.get_index(0)
            && let Some(message) = choice.get("message")
            && let Some(content) = message.get("content")
        {
            return Ok(content.as_str().unwrap_or("").to_string());
        }

        Err("Failed to extract completion from response".to_string())
    }
}

#[allow(unused)]
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
