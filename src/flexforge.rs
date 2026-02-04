//! `FlexForge` Integration for Essentia LLM Plugin
//!
//! Provides configuration panels and streaming output for LLM inference.
//!
//! ## Features
//!
//! - Provider selection (External AI, Code Assist, Local)
//! - API key configuration (secure)
//! - Streaming token output via ERSP
//! - Model parameter tuning

use essentia_traits::plugin_contracts::{
    ConfigField, ConfigSchema, FlexForgeCapability, FlexForgeIntegration, FlexForgePanelCategory,
    FlexForgePanelInfo, StreamingCapable, UiConfigurable,
};

/// LLM Plugin `FlexForge` integration.
#[derive(Debug)]
pub struct LlmPluginFlexForge {
    config:        LlmPluginConfig,
    stream_active: bool,
    stream_id:     Option<u64>,
    next_id:       u64,
}

/// Configuration for the LLM plugin.
#[derive(Debug, Clone)]
pub struct LlmPluginConfig {
    /// Selected LLM provider
    pub provider:          LlmProvider,
    /// Model name/identifier
    pub model:             String,
    /// Maximum tokens for response
    pub max_tokens:        u32,
    /// Sampling temperature
    pub temperature:       f32,
    /// Enable streaming responses
    pub streaming_enabled: bool,
    /// Request timeout in seconds
    pub timeout_secs:      u32,
    /// API endpoint (for custom providers)
    pub custom_endpoint:   Option<String>,
}

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    /// External AI API
    ExternalAI,
    /// External Code Assistance
    ExternalCodeAssist,
    /// Local Essentia SLM
    LocalSlm,
    /// Custom Essentia-compatible endpoint
    Custom,
}

impl LlmProvider {
    fn as_str(&self) -> &'static str {
        match self {
            Self::ExternalAI => "external_ai",
            Self::ExternalCodeAssist => "external_code_assist",
            Self::LocalSlm => "local_slm",
            Self::Custom => "custom",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "external_ai" => Some(Self::ExternalAI),
            "external_code_assist" => Some(Self::ExternalCodeAssist),
            "local_slm" => Some(Self::LocalSlm),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }
}

impl Default for LlmPluginConfig {
    fn default() -> Self {
        Self {
            provider:          LlmProvider::LocalSlm,
            model:             String::from("essentia-slm-100m"),
            max_tokens:        2048,
            temperature:       0.7,
            streaming_enabled: true,
            timeout_secs:      30,
            custom_endpoint:   None,
        }
    }
}

impl LlmPluginFlexForge {
    /// Creates a new `FlexForge` integration wrapper.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config:        LlmPluginConfig::default(),
            stream_active: false,
            stream_id:     None,
            next_id:       1,
        }
    }

    /// Returns panel info with capabilities.
    #[must_use]
    pub fn panel_info(&self) -> FlexForgePanelInfo {
        FlexForgePanelInfo {
            id:           self.panel_id().to_string(),
            name:         self.display_name().to_string(),
            category:     self.category(),
            icon:         self.icon_glyph().map(String::from),
            priority:     self.priority(),
            capabilities: vec![
                FlexForgeCapability::Configuration,
                FlexForgeCapability::Streaming,
            ],
        }
    }

    fn next_stream_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }
}

impl Default for LlmPluginFlexForge {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// FlexForge Integration
// ============================================================================

impl FlexForgeIntegration for LlmPluginFlexForge {
    fn panel_id(&self) -> &str {
        "essentia_llm_plugin"
    }

    fn category(&self) -> FlexForgePanelCategory {
        FlexForgePanelCategory::Intelligence
    }

    fn display_name(&self) -> &str {
        "LLM Integration"
    }

    fn icon_glyph(&self) -> Option<&str> {
        Some("\u{E8F2}") // Chat/message icon
    }

    fn priority(&self) -> u32 {
        20 // Below AI plugin in Intelligence category
    }

    fn on_panel_activate(&mut self) {
        // Initialize provider connection check if needed
    }

    fn on_panel_deactivate(&mut self) {
        if self.stream_active
            && let Some(id) = self.stream_id.take()
        {
            let _ = self.stop_stream(id);
        }
    }

    fn on_refresh(&mut self) -> bool {
        self.stream_active
    }
}

// ============================================================================
// UI Configurable
// ============================================================================

impl UiConfigurable for LlmPluginFlexForge {
    fn config_schema(&self) -> ConfigSchema {
        ConfigSchema::new()
            .with_field(
                ConfigField::select("provider", "LLM Provider", vec![
                    String::from("local_slm"),
                    String::from("external_ai"),
                    String::from("code_assist"),
                    String::from("custom"),
                ])
                .with_description("Select the LLM provider to use")
                .with_group("Provider"),
            )
            .with_field(
                ConfigField::text("model", "Model Name")
                    .with_description(
                        "Model identifier (e.g., essentia-llm-auto, essentia-slm-100m)",
                    )
                    .with_group("Provider"),
            )
            .with_field(
                ConfigField::number("max_tokens", "Max Tokens", 2048.0, 128.0, 32768.0)
                    .with_description("Maximum tokens in response")
                    .with_group("Inference"),
            )
            .with_field(
                ConfigField::number("temperature", "Temperature", 0.7, 0.0, 2.0)
                    .with_description("Sampling temperature (0 = deterministic)")
                    .with_group("Inference"),
            )
            .with_field(
                ConfigField::toggle("streaming_enabled", "Enable Streaming", true)
                    .with_description("Stream tokens as they are generated")
                    .with_group("Inference"),
            )
            .with_field(
                ConfigField::number("timeout_secs", "Timeout (seconds)", 30.0, 5.0, 300.0)
                    .with_description("Request timeout in seconds")
                    .with_group("Network"),
            )
            .with_field(
                ConfigField::text("custom_endpoint", "Custom Endpoint")
                    .with_description("Essentia-compatible API endpoint (for custom provider)")
                    .with_group("Network"),
            )
    }

    fn on_config_changed(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "provider" => {
                self.config.provider = LlmProvider::from_str(value)
                    .ok_or_else(|| format!("Unknown provider: {value}"))?;
                Ok(())
            },
            "model" => {
                if value.is_empty() {
                    return Err("Model name cannot be empty".to_string());
                }
                self.config.model = value.to_string();
                Ok(())
            },
            "max_tokens" => {
                let v: f64 = value.parse().map_err(|_| "Invalid number")?;
                self.config.max_tokens = u32::try_from(v as i64).unwrap_or(u32::MAX);
                Ok(())
            },
            "temperature" => {
                let v: f64 = value.parse().map_err(|_| "Invalid number")?;
                if !(0.0..=2.0).contains(&v) {
                    return Err("Temperature must be between 0 and 2".to_string());
                }
                self.config.temperature = v as f32;
                Ok(())
            },
            "streaming_enabled" => {
                self.config.streaming_enabled = value == "true";
                Ok(())
            },
            "timeout_secs" => {
                let v: f64 = value.parse().map_err(|_| "Invalid number")?;
                self.config.timeout_secs = u32::try_from(v as i64).unwrap_or(u32::MAX);
                Ok(())
            },
            "custom_endpoint" => {
                self.config.custom_endpoint = if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                };
                Ok(())
            },
            _ => Err(format!("Unknown configuration key: {key}")),
        }
    }

    fn apply_config(&mut self, config: &[(String, String)]) -> Result<(), String> {
        for (key, value) in config {
            self.on_config_changed(key, value)?;
        }
        Ok(())
    }

    fn get_current_config(&self) -> Vec<(String, String)> {
        vec![
            (
                String::from("provider"),
                self.config.provider.as_str().to_string(),
            ),
            (String::from("model"), self.config.model.clone()),
            (
                String::from("max_tokens"),
                self.config.max_tokens.to_string(),
            ),
            (
                String::from("temperature"),
                self.config.temperature.to_string(),
            ),
            (
                String::from("streaming_enabled"),
                self.config.streaming_enabled.to_string(),
            ),
            (
                String::from("timeout_secs"),
                self.config.timeout_secs.to_string(),
            ),
            (
                String::from("custom_endpoint"),
                self.config.custom_endpoint.clone().unwrap_or_default(),
            ),
        ]
    }

    fn reset_to_defaults(&mut self) {
        self.config = LlmPluginConfig::default();
    }
}

// ============================================================================
// Streaming Capable
// ============================================================================

impl StreamingCapable for LlmPluginFlexForge {
    fn is_streaming(&self) -> bool {
        self.stream_active
    }

    fn start_stream(&mut self) -> Result<u64, String> {
        if self.stream_active {
            return Err("Stream already active".to_string());
        }

        let stream_id = self.next_stream_id();
        self.stream_id = Some(stream_id);
        self.stream_active = true;

        Ok(stream_id)
    }

    fn stop_stream(&mut self, stream_id: u64) -> Result<(), String> {
        if !self.stream_active {
            return Err("No active stream".to_string());
        }

        if self.stream_id != Some(stream_id) {
            return Err("Invalid stream ID".to_string());
        }

        self.stream_active = false;
        self.stream_id = None;

        Ok(())
    }

    fn target_fps(&self) -> u32 {
        // Event-driven streaming, use low base rate
        10
    }

    fn render_frame(&mut self, stream_id: u64, _delta_ms: f64) -> bool {
        if !self.stream_active || self.stream_id != Some(stream_id) {
            return false;
        }

        // In production, this would emit token frames via ERSP
        // as they arrive from the LLM provider
        true
    }
}

#[cfg(all(test, feature = "full-tests"))]
mod tests {
    use super::*;

    #[test]
    fn test_panel_id() {
        let plugin = LlmPluginFlexForge::new();
        assert_eq!(plugin.panel_id(), "essentia_llm_plugin");
        assert_eq!(plugin.category(), FlexForgePanelCategory::Intelligence);
    }

    #[test]
    fn test_provider_parsing() {
        assert_eq!(
            LlmProvider::from_str("external_ai"),
            Some(LlmProvider::ExternalAI)
        );
        assert_eq!(
            LlmProvider::from_str("local_slm"),
            Some(LlmProvider::LocalSlm)
        );
        assert_eq!(LlmProvider::from_str("invalid"), None);
    }

    #[test]
    fn test_config_validation() {
        let mut plugin = LlmPluginFlexForge::new();

        // Valid provider change
        assert!(plugin.on_config_changed("provider", "external_ai").is_ok());
        assert_eq!(plugin.config.provider, LlmProvider::ExternalAI);

        // Invalid provider
        assert!(plugin.on_config_changed("provider", "invalid").is_err());

        // Empty model name
        assert!(plugin.on_config_changed("model", "").is_err());
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_streaming_lifecycle() {
        let mut plugin = LlmPluginFlexForge::new();

        let stream_id = plugin.start_stream().expect("Should start");
        assert!(plugin.is_streaming());

        plugin.stop_stream(stream_id).expect("Should stop");
        assert!(!plugin.is_streaming());
    }
}
