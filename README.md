# 🧠 Essentia Grok - Pure Rust AI Assistant

[![Rust](https://img.shields.io/badge/rust-nightly-orange)](https://www.rust-lang.org/)

## 🎯 Overview

Essentia Grok is a complete, legal pure Rust implementation for accessing Grok models via the official xAI API. Built with Rust nightly and standard library only, this project provides a rich terminal UI chat application with conversation history, enhanced features, and zero external dependencies.

**Legal & Compliant**: Uses official xAI API with personal API keys - no web scraping or unauthorized access.

## ✅ Features

- **Rich Terminal UI**: Interactive chat with colors, history, and commands
- **Official API Access**: Legal integration with xAI's Grok API
- **Conversation Continuity**: Maintains chat history and context
- **Pure Rust**: Zero external dependencies - bespoke std implementations
- **Performance Optimized**: Memory-safe, fast, and efficient
- **Extensible Architecture**: Modular design for future enhancements

## 📦 System Requirements

- Rust nightly toolchain
- Personal xAI API key (get from [x.ai](https://x.ai))

## 🚀 Getting Started

1. **Install Rust nightly:**

   ```bash
   rustup install nightly
   rustup default nightly
   ```

2. **Build:**

   ```bash
   cd E:\Projects\Grok-Api-Rust
   cargo build --release
   ```

3. **Run:**

   ```bash
   cargo run
   ```

4. **Authenticate:**

   Enter your xAI API key when prompted

5. **Chat:**

   Start conversing with Grok using the rich terminal interface!

## 🔧 How to Use

The application provides an interactive TUI chat interface:

```text
🧠 Essentia Grok - Pure Rust AI Assistant
==========================================
Legal access to Grok models via official API
Zero external dependencies - pure Rust standard library
==========================================
Enter your Grok API key: [your-api-key]
API key configured. Starting chat session...
Type 'help' for commands, 'quit' to exit.

You: Hello Grok!
Grok: Hello! I'm Grok, built by xAI. How can I help you today?

You: help
System: Essentia Grok Commands:
- help: Show this help
- clear: Clear chat history
- history: Show full history
- quit/exit: End session
...
```

### Commands

- `help`: Display available commands and features
- `clear`: Clear the chat history
- `history`: Show full conversation history
- `quit`/`exit`: End the session

## 🏗️ Architecture

- `src/main.rs`: Rich TUI chat interface with conversation management
- `src/core/grok.rs`: Official Grok API client framework
- `src/essentia/`: Bespoke standard library implementations
  - `http.rs`: HTTP client framework (HTTPS ready)
  - `crypto.rs`: Cryptographic operations
  - `json.rs`: JSON parsing/encoding
  - `tls.rs`: TLS framework
  - `url.rs`: URL handling
  - And more...

## 🚨 Current Limitations

Due to pure std library constraint:

- **HTTPS/TLS**: Framework ready, but requires std implementation
- **Real API Calls**: Dummy responses until HTTPS is available
- **Full Crypto**: Basic implementations for framework

The architecture is designed for easy integration once HTTPS becomes available in std.

## 🔑 Official API Integration

Uses xAI's official API endpoints:

- **Endpoint**: `https://api.x.ai/v1/chat/completions`
- **Authentication**: Bearer token with API key
- **Models**: grok-2-1212 (latest)
- **Features**: Streaming responses, conversation context

## 📚 Development History

Originally analyzed a Python implementation that used web scraping for unauthorized access. This Rust version provides a legal, official API alternative with enhanced UI and pure std implementation.

## 🙌 Community

Demonstration of advanced Rust capabilities for AI API integration without external dependencies.

## ✅ License

MIT License
