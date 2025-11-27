//! # Essentia LLM Plugin
//!
//! Pure std-only implementation for LLM API integration (Grok, Copilot).
//! Zero third-party dependencies - SSOP compliant.
//!
//! ## Features
//! - Base64 encoding/decoding
//! - JSON serialization (bespoke)
//! - HTTP client (minimal)
//! - TLS handshake (std-only)
//! - Regex matching (simplified)
//! - UUID v4 generation
//! - HTML parsing
//! - Cookie management
//!
//! ## `FlexForge` Integration
//!
//! The LLM plugin integrates with `FlexForge` via `LlmPluginFlexForge`:
//!
//! ```ignore
//! use essentia_llm_plugin::flexforge::LlmPluginFlexForge;
//!
//! let plugin = LlmPluginFlexForge::new();
//! let panel_info = plugin.panel_info();
//! ```

// TODO: Add comprehensive documentation to all public items
// Tracked in documentation remediation queue
#![allow(missing_docs)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod core;
pub mod essentia;
pub mod flexforge;

// Re-exports for convenience
pub use flexforge::{LlmPluginConfig, LlmPluginFlexForge, LlmProvider};
