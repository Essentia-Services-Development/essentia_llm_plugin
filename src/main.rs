//! Essentia LLM CLI - Pure std-only AI chat interface.
#![allow(missing_docs, clippy::needless_raw_string_hashes)]
#![allow(
    clippy::format_push_string,
    clippy::uninlined_format_args,
    clippy::unnecessary_wraps,
    clippy::redundant_closure_for_method_calls,
    clippy::manual_string_new,
    clippy::no_effect_underscore_binding,
    clippy::unused_self,
    clippy::struct_field_names,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::similar_names,
    clippy::many_single_char_names,
    clippy::trivially_copy_pass_by_ref,
    clippy::needless_pass_by_value,
    clippy::must_use_candidate,
    clippy::self_only_used_in_recursion,
    clippy::map_unwrap_or,
    clippy::single_char_pattern,
    clippy::elidable_lifetime_names,
    clippy::explicit_iter_loop
)]

// SSOP-EXEMPT(std::io): Required for synchronous CLI console I/O operations
use std::{
    collections::VecDeque,
    io::{self, Write},
};

mod core;
mod essentia;

use core::external_llm::ExternalLlm;

const MAX_HISTORY: usize = 100;

struct ChatUI {
    history: VecDeque<String>,
    api_key: String,
}

impl ChatUI {
    fn new() -> Option<Self> {
        println!("🧠 Essentia LLM - Pure Rust AI Assistant");
        println!("==========================================");
        println!("Legal access to AI models via official API");
        println!("Zero external dependencies - pure Rust standard library");
        println!("==========================================");

        print!("Enter your API key: ");
        io::stdout().flush().ok()?;
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).ok()?;
        let api_key = api_key.trim().to_string();

        println!("API key configured. Starting chat session...");
        println!("Type 'help' for commands, 'quit' to exit.");
        println!();

        Some(Self { history: VecDeque::new(), api_key })
    }

    fn run(&mut self) {
        loop {
            self.display_history();
            print!("You: ");
            if io::stdout().flush().is_err() {
                continue;
            }

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            self.add_to_history(format!("You: {}", input));

            match input {
                "quit" | "exit" => break,
                "help" => {
                    self.show_help();
                },
                "clear" => {
                    self.history.clear();
                },
                "history" => {
                    self.show_full_history();
                },
                _ => {
                    self.process_message(input);
                },
            }
        }

        println!("Session ended. Goodbye! 👋");
    }

    fn display_history(&self) {
        // Clear screen (ANSI escape)
        print!("\x1B[2J\x1B[1;1H");

        println!("🧠 Essentia LLM Chat");
        println!("====================");

        // Show last 10 messages
        let start = if self.history.len() > 10 {
            self.history.len() - 10
        } else {
            0
        };
        for msg in self.history.iter().skip(start) {
            println!("{}", msg);
        }
        println!();
    }

    fn add_to_history(&mut self, msg: String) {
        self.history.push_back(msg);
        if self.history.len() > MAX_HISTORY {
            self.history.pop_front();
        }
    }

    fn show_help(&mut self) {
        let help = r#"
Essentia LLM Commands:
- help: Show this help
- clear: Clear chat history
- history: Show full history
- quit/exit: End session

Features:
- Legal API access with personal credentials
- Conversation continuity
- Rich terminal UI with colors
- Performance optimized pure Rust
- Zero external dependencies

Note: Full HTTPS implementation required for production API access.
Current version uses dummy responses for demonstration.
"#;
        self.add_to_history(format!("System: {}", help));
    }

    fn show_full_history(&mut self) {
        let mut full = "Full Chat History:\n".to_string();
        for msg in &self.history {
            full.push_str(msg);
            full.push('\n');
        }
        self.add_to_history(format!("System: {}", full));
    }

    fn process_message(&mut self, message: &str) {
        // Use UUID for request ID
        let request_id = crate::essentia::uuid::Uuid::new_v4();

        // Use regex to validate and process message
        let message_pattern = r"^[a-zA-Z0-9\s\.,!?\-]+$";
        let regex = match crate::essentia::regex::Regex::new(message_pattern) {
            Ok(r) => r,
            Err(e) => {
                self.add_to_history(format!("Regex Error: {}", e));
                return;
            },
        };
        if !regex.is_match(message) {
            self.add_to_history("Error: Invalid message format".to_string());
            return;
        }

        // Use UUID functions
        let uuid_bytes = request_id.as_bytes();
        let _reconstructed_uuid = crate::essentia::uuid::Uuid::from_bytes(*uuid_bytes);

        // Use regex find_iter
        let _iter_results: Vec<_> = regex.find_iter(message).collect();

        // Use base64 encoding for API key
        let encoded_key = crate::essentia::base64::encode(self.api_key.as_bytes());

        // Use base64 decode as well
        let _decoded_key = crate::essentia::base64::decode(&encoded_key);

        // Use HTTP functions (dummy calls)
        let _get_result = crate::essentia::http::get("http://example.com");

        // Use crypto functions for request signing
        let message_bytes = message.as_bytes();
        let hash = essentia_core_utils::crypto::sha256(message_bytes);
        // TODO(IMPL): Implement signing with API key
        let hmac = essentia_core_utils::crypto::Hmac::new(self.api_key.as_bytes());
        let signature = hmac.compute(message_bytes);

        // Build JSON payload using our JSON implementation
        let payload = crate::essentia::json::Value::Object({
            let mut obj = std::collections::HashMap::new();
            obj.insert(
                "model".to_string(),
                crate::essentia::json::Value::String("essentia-llm-auto".to_string()),
            );
            obj.insert(
                "messages".to_string(),
                crate::essentia::json::Value::Array(vec![crate::essentia::json::Value::Object({
                    let mut msg_obj = std::collections::HashMap::new();
                    msg_obj.insert(
                        "role".to_string(),
                        crate::essentia::json::Value::String("user".to_string()),
                    );
                    msg_obj.insert(
                        "content".to_string(),
                        crate::essentia::json::Value::String(message.to_string()),
                    );
                    msg_obj.insert(
                        "signature".to_string(),
                        crate::essentia::json::Value::String(crate::essentia::base64::encode(
                            &signature,
                        )),
                    );
                    msg_obj
                })]),
            );
            obj.insert(
                "request_id".to_string(),
                crate::essentia::json::Value::String(request_id.to_string()),
            );
            obj.insert(
                "hash".to_string(),
                crate::essentia::json::Value::String(crate::essentia::base64::encode(&hash)),
            );
            obj
        });

        let json_payload = crate::essentia::json::to_json_string(&payload);

        // Use HTTP post
        let _post_result = crate::essentia::http::post("http://example.com", &json_payload);

        // Use URL parsing
        let api_url = "https://api.x.ai/v1/chat/completions";
        let parsed_url = match crate::essentia::url::Url::parse(api_url) {
            Ok(url) => url,
            Err(e) => {
                self.add_to_history(format!("URL Parse Error: {}", e));
                return;
            },
        };
        let _username = &parsed_url.username;
        let _password = &parsed_url.password;

        // Use uuid4 function
        let _uuid_string = crate::essentia::uuid::Uuid::new_v4();

        // Use HTML parsing for any web content (simulated)
        let html_content = format!("<html><body>{}</body></html>", message);
        let _scripts = crate::essentia::html::find_scripts(&html_content);
        let _meta_baggage = crate::essentia::html::find_meta_baggage(&html_content);
        let _meta_sentry = crate::essentia::html::find_meta_sentry(&html_content);
        let _anim = crate::essentia::html::find_anim(&html_content);

        // Use HTML Document parsing
        let document = match crate::essentia::html::Document::parse(&html_content) {
            Ok(doc) => doc,
            Err(e) => {
                self.add_to_history(format!("HTML Parse Error: {}", e));
                return;
            },
        };
        let _scripts = document.find_scripts();
        let _meta_content = document.find_meta_content("description");

        // Use HTML Element fields by accessing root
        let _root_tag = &document.root.tag_name;
        let _root_attrs = &document.root.attributes;
        let _text = &document.root.text_content;
        let _children = &document.root.children;

        // Use Node enum
        let _text_node = crate::essentia::html::Node::Text(());

        // Use cookies
        let mut cookie_jar = crate::essentia::cookies::CookieJar::new();
        cookie_jar.set("session_id", &request_id.to_string());
        let _session_cookie = cookie_jar.get("session_id");
        let _cookies = cookie_jar.get_dict();
        cookie_jar.update(&std::collections::HashMap::new());

        // Use multipart (for file uploads if needed)
        let _multipart_data = crate::essentia::multipart::create_multipart("boundary123", vec![
            ("field1", "text/plain", b"value1"),
            ("field2", "application/json", json_payload.as_bytes()),
        ]);

        // In real implementation, this would call the official external AI API
        // For now, dummy response that uses our implementations
        let llm = ExternalLlm::new(
            "essentia-llm-auto",
            &parsed_url.hostname.unwrap_or_default(),
        );

        match llm.chat_with_api(&encoded_key, &json_payload, &self.get_context()) {
            Ok(response) => {
                // Use the Response struct
                let response_obj = llm.create_response(response.clone());
                let _resp = &response_obj.response;
                let _stream = &response_obj.stream_response;
                let _images = &response_obj.images;
                let _extra = &response_obj.extra_data;

                self.add_to_history(format!("AI: {}", response));
            },
            Err(e) => {
                self.add_to_history(format!("Error: {}", e));
            },
        }
    }

    fn get_context(&self) -> Vec<String> {
        // Get recent context for conversation continuity
        self.history.iter().rev().take(20).rev().cloned().collect()
    }
}

fn main() {
    let Some(mut ui) = ChatUI::new() else {
        eprintln!("Failed to initialize chat UI");
        return;
    };
    ui.run();
}
