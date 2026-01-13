//! Claude Code CLI integration module.
//!
//! Provides functionality for:
//! - Claude Code CLI process management
//! - Output parsing for tasks/progress
//! - Response summarization and TTS
//! - Voice input (STT) integration
//! - Custom Stream Deck button layouts

pub mod actions;
pub mod buttons;
pub mod cli;
pub mod monitor;
pub mod speech;

pub use buttons::{get_buttons_for_layout, ButtonLayout};
pub use cli::ClaudeCodeManager;
pub use monitor::ProgressMonitor;
pub use speech::{SpeechToText, TextToSpeech};
