#[allow(dead_code)]
#[derive(Clone)]
pub struct CopilotModels;

#[allow(dead_code)]
impl CopilotModels {
    pub fn get_available_models() -> Vec<&'static str> {
        vec!["gpt-4", "gpt-3.5-turbo", "gpt-4-turbo", "gpt-4o"]
    }
}

#[allow(dead_code)]
pub struct Copilot {
    model: String,
}

#[allow(dead_code)]
impl Copilot {
    pub fn new(model: &str) -> Self {
        Self { model: model.to_string() }
    }

    pub fn chat_with_api(
        &self, github_token: &str, message: &str, context: &[String],
    ) -> Result<String, String> {
        // GitHub Copilot API endpoint for completions
        let url = "https://api.github.com/copilot_internal/v2/completions";

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
            crate::essentia::http::post_with_auth(url, &format!("Bearer {}", github_token), &body)
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
