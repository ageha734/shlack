//! # Shlack
//!
//! A command line utility to pipe text into Slack messages, written in Rust.
//!
//! ## Features
//!
//! - 🚀 Fast and lightweight Slack messaging client
//! - 🔧 Easy to use command-line interface
//! - 🧪 Comprehensive test coverage
//! - 🤖 Automated CI/CD pipeline with GitHub Actions
//! - 📦 Cross-platform releases (Linux, Windows, macOS)
//!
//! ## Usage
//!
//! ```rust
//! use shlack::Msg;
//!
//! let msg = Msg::new(
//!     "Hello, World!".to_string(),
//!     "general".to_string(),
//!     "your-slack-token".to_string(),
//! );
//!
//! // Send the message (requires valid Slack token)
//! // msg.send().expect("Failed to send message");
//! ```
//!
//! ## Command Line Usage
//!
//! ```bash
//! echo "Hello, World!" | shlack general
//! ```

pub mod msg;

pub use msg::{Msg, HasId, HasName};
