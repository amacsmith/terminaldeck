//! Claude-specific Stream Deck button layouts.

use crate::streamdeck::ButtonConfig;
use serde::{Deserialize, Serialize};
use specta::Type;

/// Available button layout modes
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default, PartialEq)]
pub enum ButtonLayout {
    #[default]
    /// Default terminal control layout
    Default,
    /// Claude Code focused layout with STT/TTS
    ClaudeCode,
    /// Project building and management
    ProjectBuild,
    /// Numbered response mode (1-9 quick responses)
    NumberedResponse,
    /// Monitoring and progress view
    Monitoring,
}

impl ButtonLayout {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "claude" | "claude_code" | "claudecode" => ButtonLayout::ClaudeCode,
            "project" | "build" | "project_build" => ButtonLayout::ProjectBuild,
            "numbered" | "numbers" | "response" => ButtonLayout::NumberedResponse,
            "monitor" | "monitoring" | "progress" => ButtonLayout::Monitoring,
            _ => ButtonLayout::Default,
        }
    }
}

/// Get buttons for the Claude Code layout
/// Layout:
/// Row 1: [STT] [TTS] [Terminal 1] [Terminal 2] [Next Term]
/// Row 2: [1.] [2.] [3.] [4.] [Submit]
/// Row 3: [Yes] [No] [Stop] [Build] [Test]
pub fn get_claude_code_buttons() -> Vec<ButtonConfig> {
    vec![
        // Row 1: Voice & Terminal Control
        ButtonConfig {
            id: 0,
            label: "Voice".into(),
            sublabel: Some("STT".into()),
            color: "#5a4a78".into(), // Purple
            action: "claude_stt_toggle".into(),
        },
        ButtonConfig {
            id: 1,
            label: "Speak".into(),
            sublabel: Some("TTS".into()),
            color: "#5a4a78".into(),
            action: "claude_tts_toggle".into(),
        },
        ButtonConfig {
            id: 2,
            label: "Term".into(),
            sublabel: Some("1".into()),
            color: "#3d5a80".into(), // Blue
            action: "claude_terminal_1".into(),
        },
        ButtonConfig {
            id: 3,
            label: "Term".into(),
            sublabel: Some("2".into()),
            color: "#3d5a80".into(),
            action: "claude_terminal_2".into(),
        },
        ButtonConfig {
            id: 4,
            label: "Next".into(),
            sublabel: Some("→".into()),
            color: "#2d3748".into(), // Gray
            action: "claude_next_terminal".into(),
        },

        // Row 2: Numbered Responses
        ButtonConfig {
            id: 5,
            label: "1".into(),
            sublabel: None,
            color: "#3d6b59".into(), // Green
            action: "claude_respond_1".into(),
        },
        ButtonConfig {
            id: 6,
            label: "2".into(),
            sublabel: None,
            color: "#3d6b59".into(),
            action: "claude_respond_2".into(),
        },
        ButtonConfig {
            id: 7,
            label: "3".into(),
            sublabel: None,
            color: "#3d6b59".into(),
            action: "claude_respond_3".into(),
        },
        ButtonConfig {
            id: 8,
            label: "4".into(),
            sublabel: None,
            color: "#3d6b59".into(),
            action: "claude_respond_4".into(),
        },
        ButtonConfig {
            id: 9,
            label: "Submit".into(),
            sublabel: Some("↵".into()),
            color: "#3d6b59".into(),
            action: "submit".into(),
        },

        // Row 3: Common Actions
        ButtonConfig {
            id: 10,
            label: "Yes".into(),
            sublabel: Some("Y".into()),
            color: "#3d6b59".into(),
            action: "yes".into(),
        },
        ButtonConfig {
            id: 11,
            label: "No".into(),
            sublabel: Some("N".into()),
            color: "#8b3a3a".into(), // Red
            action: "no".into(),
        },
        ButtonConfig {
            id: 12,
            label: "Stop".into(),
            sublabel: Some("^C".into()),
            color: "#8b3a3a".into(),
            action: "claude_stop".into(),
        },
        ButtonConfig {
            id: 13,
            label: "Build".into(),
            sublabel: Some("/build".into()),
            color: "#3d5a80".into(),
            action: "claude_build".into(),
        },
        ButtonConfig {
            id: 14,
            label: "Test".into(),
            sublabel: Some("/test".into()),
            color: "#3d5a80".into(),
            action: "claude_test".into(),
        },
    ]
}

/// Get buttons for the Project Build layout
/// Layout:
/// Row 1: [Build] [Test] [Run] [Commit] [PR]
/// Row 2: [Yes] [No] [Cancel] [1] [2]
/// Row 3: [STT] [Submit] [UP] [DOWN] [Delete]
pub fn get_project_build_buttons() -> Vec<ButtonConfig> {
    vec![
        // Row 1: Project Commands
        ButtonConfig {
            id: 0,
            label: "Build".into(),
            sublabel: Some("/build".into()),
            color: "#3d5a80".into(),
            action: "claude_build".into(),
        },
        ButtonConfig {
            id: 1,
            label: "Test".into(),
            sublabel: Some("/test".into()),
            color: "#3d5a80".into(),
            action: "claude_test".into(),
        },
        ButtonConfig {
            id: 2,
            label: "Run".into(),
            sublabel: None,
            color: "#3d6b59".into(),
            action: "claude_run".into(),
        },
        ButtonConfig {
            id: 3,
            label: "Commit".into(),
            sublabel: Some("/commit".into()),
            color: "#5a4a78".into(),
            action: "claude_commit".into(),
        },
        ButtonConfig {
            id: 4,
            label: "PR".into(),
            sublabel: Some("Create".into()),
            color: "#5a4a78".into(),
            action: "claude_pr".into(),
        },

        // Row 2: Responses
        ButtonConfig {
            id: 5,
            label: "Yes".into(),
            sublabel: Some("Y".into()),
            color: "#3d6b59".into(),
            action: "yes".into(),
        },
        ButtonConfig {
            id: 6,
            label: "No".into(),
            sublabel: Some("N".into()),
            color: "#8b3a3a".into(),
            action: "no".into(),
        },
        ButtonConfig {
            id: 7,
            label: "Cancel".into(),
            sublabel: Some("ESC".into()),
            color: "#8b3a3a".into(),
            action: "cancel".into(),
        },
        ButtonConfig {
            id: 8,
            label: "1".into(),
            sublabel: None,
            color: "#3d6b59".into(),
            action: "claude_respond_1".into(),
        },
        ButtonConfig {
            id: 9,
            label: "2".into(),
            sublabel: None,
            color: "#3d6b59".into(),
            action: "claude_respond_2".into(),
        },

        // Row 3: Input Control
        ButtonConfig {
            id: 10,
            label: "Voice".into(),
            sublabel: Some("STT".into()),
            color: "#5a4a78".into(),
            action: "dictate".into(),
        },
        ButtonConfig {
            id: 11,
            label: "Submit".into(),
            sublabel: Some("↵".into()),
            color: "#3d6b59".into(),
            action: "submit".into(),
        },
        ButtonConfig {
            id: 12,
            label: "UP".into(),
            sublabel: None,
            color: "#2d3748".into(),
            action: "up".into(),
        },
        ButtonConfig {
            id: 13,
            label: "DOWN".into(),
            sublabel: None,
            color: "#2d3748".into(),
            action: "down".into(),
        },
        ButtonConfig {
            id: 14,
            label: "Delete".into(),
            sublabel: Some("DEL".into()),
            color: "#2d3748".into(),
            action: "delete".into(),
        },
    ]
}

/// Get buttons for the Numbered Response layout
/// Full grid of 1-9 plus common actions
pub fn get_numbered_response_buttons() -> Vec<ButtonConfig> {
    vec![
        // Row 1: 1-5
        ButtonConfig { id: 0, label: "1".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_1".into() },
        ButtonConfig { id: 1, label: "2".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_2".into() },
        ButtonConfig { id: 2, label: "3".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_3".into() },
        ButtonConfig { id: 3, label: "4".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_4".into() },
        ButtonConfig { id: 4, label: "5".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_5".into() },

        // Row 2: 6-9 + Submit
        ButtonConfig { id: 5, label: "6".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_6".into() },
        ButtonConfig { id: 6, label: "7".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_7".into() },
        ButtonConfig { id: 7, label: "8".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_8".into() },
        ButtonConfig { id: 8, label: "9".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_respond_9".into() },
        ButtonConfig { id: 9, label: "Submit".into(), sublabel: Some("↵".into()), color: "#3d5a80".into(), action: "submit".into() },

        // Row 3: Control actions
        ButtonConfig { id: 10, label: "Yes".into(), sublabel: Some("Y".into()), color: "#3d6b59".into(), action: "yes".into() },
        ButtonConfig { id: 11, label: "No".into(), sublabel: Some("N".into()), color: "#8b3a3a".into(), action: "no".into() },
        ButtonConfig { id: 12, label: "Cancel".into(), sublabel: Some("ESC".into()), color: "#8b3a3a".into(), action: "cancel".into() },
        ButtonConfig { id: 13, label: "Voice".into(), sublabel: Some("STT".into()), color: "#5a4a78".into(), action: "dictate".into() },
        ButtonConfig { id: 14, label: "Back".into(), sublabel: Some("←".into()), color: "#2d3748".into(), action: "layout_default".into() },
    ]
}

/// Get buttons for the Monitoring layout
/// Shows progress indicators and control actions
pub fn get_monitoring_buttons() -> Vec<ButtonConfig> {
    vec![
        // Row 1: Status & Phase
        ButtonConfig { id: 0, label: "Tasks".into(), sublabel: Some("📋".into()), color: "#3d5a80".into(), action: "claude_show_tasks".into() },
        ButtonConfig { id: 1, label: "Progress".into(), sublabel: Some("📊".into()), color: "#3d5a80".into(), action: "claude_show_progress".into() },
        ButtonConfig { id: 2, label: "Logs".into(), sublabel: Some("📜".into()), color: "#3d5a80".into(), action: "claude_show_logs".into() },
        ButtonConfig { id: 3, label: "Phase".into(), sublabel: Some("🎯".into()), color: "#3d5a80".into(), action: "claude_show_phase".into() },
        ButtonConfig { id: 4, label: "Refresh".into(), sublabel: Some("🔄".into()), color: "#2d3748".into(), action: "claude_refresh".into() },

        // Row 2: Quick Actions
        ButtonConfig { id: 5, label: "Resume".into(), sublabel: None, color: "#3d6b59".into(), action: "claude_resume".into() },
        ButtonConfig { id: 6, label: "Stop".into(), sublabel: Some("^C".into()), color: "#8b3a3a".into(), action: "claude_stop".into() },
        ButtonConfig { id: 7, label: "New".into(), sublabel: Some("Chat".into()), color: "#5a4a78".into(), action: "claude_new_chat".into() },
        ButtonConfig { id: 8, label: "Yes".into(), sublabel: Some("Y".into()), color: "#3d6b59".into(), action: "yes".into() },
        ButtonConfig { id: 9, label: "No".into(), sublabel: Some("N".into()), color: "#8b3a3a".into(), action: "no".into() },

        // Row 3: Navigation
        ButtonConfig { id: 10, label: "Voice".into(), sublabel: Some("STT".into()), color: "#5a4a78".into(), action: "dictate".into() },
        ButtonConfig { id: 11, label: "Speak".into(), sublabel: Some("TTS".into()), color: "#5a4a78".into(), action: "claude_tts_toggle".into() },
        ButtonConfig { id: 12, label: "Layout".into(), sublabel: Some("Codes".into()), color: "#2d3748".into(), action: "layout_claude".into() },
        ButtonConfig { id: 13, label: "Layout".into(), sublabel: Some("Build".into()), color: "#2d3748".into(), action: "layout_build".into() },
        ButtonConfig { id: 14, label: "Default".into(), sublabel: Some("←".into()), color: "#2d3748".into(), action: "layout_default".into() },
    ]
}

/// Get buttons for the specified layout
pub fn get_buttons_for_layout(layout: &ButtonLayout) -> Vec<ButtonConfig> {
    match layout {
        ButtonLayout::Default => crate::streamdeck::get_default_buttons(),
        ButtonLayout::ClaudeCode => get_claude_code_buttons(),
        ButtonLayout::ProjectBuild => get_project_build_buttons(),
        ButtonLayout::NumberedResponse => get_numbered_response_buttons(),
        ButtonLayout::Monitoring => get_monitoring_buttons(),
    }
}
