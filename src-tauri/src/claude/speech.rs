//! Speech integration: Text-to-Speech and Speech-to-Text.

use log::{debug, error, info};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

static TTS_ENABLED: AtomicBool = AtomicBool::new(true);
static STT_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Text-to-Speech configuration
pub struct TextToSpeech {
    /// Voice to use (macOS voices: Alex, Samantha, etc.)
    pub voice: String,
    /// Speech rate (words per minute, default ~175)
    pub rate: u16,
    /// Whether TTS is enabled
    pub enabled: bool,
}

impl Default for TextToSpeech {
    fn default() -> Self {
        TextToSpeech {
            voice: "Samantha".to_string(),
            rate: 200,
            enabled: true,
        }
    }
}

impl TextToSpeech {
    /// Speak text using macOS `say` command
    pub fn speak(&self, text: &str) -> Result<(), String> {
        if !self.enabled || !TTS_ENABLED.load(Ordering::SeqCst) {
            debug!("TTS disabled, skipping: {}", &text[..text.len().min(50)]);
            return Ok(());
        }

        // Sanitize text for shell
        let clean_text = text
            .replace('"', r#"\""#)
            .replace('`', "'")
            .replace('\n', " ");

        let output = Command::new("say")
            .arg("-v")
            .arg(&self.voice)
            .arg("-r")
            .arg(self.rate.to_string())
            .arg(&clean_text)
            .output()
            .map_err(|e| format!("Failed to run say command: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("TTS failed: {stderr}");
            return Err(format!("TTS error: {stderr}"));
        }

        info!("Spoke: {}...", &text[..text.len().min(50)]);
        Ok(())
    }

    /// Speak a concise summary of numbered options
    pub fn speak_options(&self, options: &[String]) -> Result<(), String> {
        if options.is_empty() {
            return Ok(());
        }

        let summary = if options.len() <= 4 {
            options
                .iter()
                .enumerate()
                .map(|(i, opt)| format!("{}. {}", i + 1, opt))
                .collect::<Vec<_>>()
                .join(". ")
        } else {
            format!(
                "There are {} options. First three are: {}",
                options.len(),
                options[..3]
                    .iter()
                    .enumerate()
                    .map(|(i, opt)| format!("{}. {}", i + 1, opt))
                    .collect::<Vec<_>>()
                    .join(". ")
            )
        };

        self.speak(&summary)
    }

    /// Enable/disable TTS globally
    pub fn set_enabled(enabled: bool) {
        TTS_ENABLED.store(enabled, Ordering::SeqCst);
        info!("TTS {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Check if TTS is enabled
    pub fn is_enabled() -> bool {
        TTS_ENABLED.load(Ordering::SeqCst)
    }
}

/// Speech-to-Text integration
pub struct SpeechToText {
    /// Dictation shortcut method
    pub method: SttMethod,
}

/// STT activation method
#[derive(Debug, Clone)]
pub enum SttMethod {
    /// Press Fn/Globe key twice (macOS Dictation)
    FnTwice,
    /// Hold Fn/Globe key (Wispr Flow)
    FnHold,
    /// Press Control key twice
    CtrlTwice,
    /// Custom keyboard shortcut (e.g., "cmd+shift+d")
    Custom(String),
}

impl Default for SpeechToText {
    fn default() -> Self {
        SpeechToText {
            method: SttMethod::CtrlTwice,
        }
    }
}

impl SpeechToText {
    /// Activate speech-to-text input
    pub fn activate(&self) -> Result<(), String> {
        if STT_ACTIVE.load(Ordering::SeqCst) {
            debug!("STT already active");
            return Ok(());
        }

        let script = match &self.method {
            SttMethod::FnTwice => r#"
                tell application "System Events"
                    key code 63
                    delay 0.15
                    key code 63
                end tell
            "#.to_string(),
            SttMethod::FnHold => r#"
                tell application "System Events"
                    key down 63
                    delay 0.5
                    key up 63
                end tell
            "#.to_string(),
            SttMethod::CtrlTwice => r#"
                tell application "System Events"
                    key down 59
                    delay 0.05
                    key up 59
                    delay 0.3
                    key down 59
                    delay 0.05
                    key up 59
                end tell
            "#.to_string(),
            SttMethod::Custom(shortcut) => {
                parse_custom_shortcut(shortcut).unwrap_or_else(|| {
                    // Fallback to ctrl twice
                    r#"
                        tell application "System Events"
                            key code 59
                            delay 0.2
                            key code 59
                        end tell
                    "#.to_string()
                })
            }
        };

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| format!("Failed to activate STT: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("STT activation failed: {stderr}"));
        }

        STT_ACTIVE.store(true, Ordering::SeqCst);
        info!("STT activated");
        Ok(())
    }

    /// Deactivate speech-to-text (toggle off)
    pub fn deactivate(&self) -> Result<(), String> {
        if !STT_ACTIVE.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Same key press to toggle off
        self.activate()?;
        STT_ACTIVE.store(false, Ordering::SeqCst);
        info!("STT deactivated");
        Ok(())
    }

    /// Check if STT is currently active
    pub fn is_active() -> bool {
        STT_ACTIVE.load(Ordering::SeqCst)
    }
}

/// Parse a custom shortcut string into AppleScript
fn parse_custom_shortcut(shortcut: &str) -> Option<String> {
    let parts: Vec<String> = shortcut
        .split('+')
        .map(|s| s.trim().to_lowercase())
        .collect();

    if parts.is_empty() {
        return None;
    }

    let key = parts.last()?;
    let modifiers: Vec<&str> = parts[..parts.len() - 1]
        .iter()
        .map(|s| s.as_str())
        .collect();

    let mut modifier_str = Vec::new();
    for m in &modifiers {
        match *m {
            "cmd" | "command" => modifier_str.push("command down"),
            "ctrl" | "control" => modifier_str.push("control down"),
            "alt" | "option" => modifier_str.push("option down"),
            "shift" => modifier_str.push("shift down"),
            _ => {}
        }
    }

    let modifiers_applescript = if modifier_str.is_empty() {
        String::new()
    } else {
        format!(" using {{{}}}", modifier_str.join(", "))
    };

    Some(format!(
        r#"tell application "System Events" to keystroke "{}"{}"#,
        key, modifiers_applescript
    ))
}
