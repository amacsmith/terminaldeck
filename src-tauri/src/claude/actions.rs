//! Claude-specific Stream Deck actions.

use super::speech::{SpeechToText, TextToSpeech};
use log::{debug, error, info};
use std::process::Command;

/// Execute Claude-specific actions from Stream Deck
pub fn execute_claude_action(action: &str, settings: &ClaudeActionSettings) -> Result<(), String> {
    info!("Executing Claude action: {action}");

    match action {
        // Voice Actions
        "claude_stt_toggle" => toggle_stt(settings),
        "claude_tts_toggle" => toggle_tts(),
        "claude_tts_repeat" => repeat_last_tts(settings),

        // Response Actions
        "claude_respond_1" => send_numbered_response(1),
        "claude_respond_2" => send_numbered_response(2),
        "claude_respond_3" => send_numbered_response(3),
        "claude_respond_4" => send_numbered_response(4),

        // Navigation Actions
        "claude_next_terminal" => switch_to_next_terminal(&settings.terminal_app),
        "claude_prev_terminal" => switch_to_prev_terminal(&settings.terminal_app),
        "claude_terminal_1" => switch_to_terminal_number(1, &settings.terminal_app),
        "claude_terminal_2" => switch_to_terminal_number(2, &settings.terminal_app),
        "claude_terminal_3" => switch_to_terminal_number(3, &settings.terminal_app),
        "claude_terminal_4" => switch_to_terminal_number(4, &settings.terminal_app),

        // Project Actions
        "claude_build" => send_project_command("build"),
        "claude_test" => send_project_command("test"),
        "claude_run" => send_project_command("run"),
        "claude_commit" => send_project_command("commit"),

        // Session Actions
        "claude_resume" => resume_claude_session(),
        "claude_stop" => stop_claude_session(),
        "claude_new_chat" => start_new_chat(),

        // Monitoring Actions
        "claude_show_tasks" => show_task_summary(settings),
        "claude_show_progress" => show_progress_summary(settings),

        _ => {
            error!("Unknown Claude action: {action}");
            Err(format!("Unknown Claude action: {action}"))
        }
    }
}

/// Settings for Claude actions
#[derive(Debug, Clone)]
pub struct ClaudeActionSettings {
    pub terminal_app: String,
    pub tts_enabled: bool,
    pub tts_voice: String,
    pub stt_method: String,
    pub last_tts_text: String,
}

impl Default for ClaudeActionSettings {
    fn default() -> Self {
        ClaudeActionSettings {
            terminal_app: "Terminal".to_string(),
            tts_enabled: true,
            tts_voice: "Samantha".to_string(),
            stt_method: "ctrl_twice".to_string(),
            last_tts_text: String::new(),
        }
    }
}

// ============================================================================
// Voice Actions
// ============================================================================

fn toggle_stt(settings: &ClaudeActionSettings) -> Result<(), String> {
    let stt = SpeechToText::default();
    if SpeechToText::is_active() {
        stt.deactivate()
    } else {
        stt.activate()
    }
}

fn toggle_tts() -> Result<(), String> {
    let enabled = TextToSpeech::is_enabled();
    TextToSpeech::set_enabled(!enabled);

    if !enabled {
        // Just enabled, confirm with speech
        let tts = TextToSpeech::default();
        tts.speak("Voice enabled")?;
    }

    Ok(())
}

fn repeat_last_tts(settings: &ClaudeActionSettings) -> Result<(), String> {
    if settings.last_tts_text.is_empty() {
        return Ok(());
    }

    let tts = TextToSpeech {
        voice: settings.tts_voice.clone(),
        ..Default::default()
    };
    tts.speak(&settings.last_tts_text)
}

// ============================================================================
// Response Actions
// ============================================================================

fn send_numbered_response(number: u8) -> Result<(), String> {
    // Type the number and press Enter
    let script = format!(
        r#"tell application "System Events"
            keystroke "{number}"
            delay 0.1
            key code 36
        end tell"#
    );
    run_applescript(&script)
}

// ============================================================================
// Terminal Navigation
// ============================================================================

fn switch_to_next_terminal(terminal_app: &str) -> Result<(), String> {
    let app = normalize_terminal_name(terminal_app);
    let script = format!(
        r#"tell application "{app}"
            activate
        end tell
        tell application "System Events"
            keystroke "`" using command down
        end tell"#
    );
    run_applescript(&script)
}

fn switch_to_prev_terminal(terminal_app: &str) -> Result<(), String> {
    let app = normalize_terminal_name(terminal_app);
    let script = format!(
        r#"tell application "{app}"
            activate
        end tell
        tell application "System Events"
            keystroke "`" using {{command down, shift down}}
        end tell"#
    );
    run_applescript(&script)
}

fn switch_to_terminal_number(number: u8, terminal_app: &str) -> Result<(), String> {
    let app = normalize_terminal_name(terminal_app);

    // Use Cmd+Number to switch tabs/windows
    let script = format!(
        r#"tell application "{app}"
            activate
        end tell
        tell application "System Events"
            keystroke "{number}" using command down
        end tell"#
    );
    run_applescript(&script)
}

// ============================================================================
// Project Commands
// ============================================================================

fn send_project_command(command: &str) -> Result<(), String> {
    // Type the command to Claude Code
    let cmd = match command {
        "build" => "/build",
        "test" => "/test",
        "run" => "run the project",
        "commit" => "/commit",
        _ => command,
    };

    // Type command and submit
    let script = format!(
        r#"tell application "System Events"
            keystroke "{cmd}"
            delay 0.1
            key code 36
        end tell"#
    );
    run_applescript(&script)
}

// ============================================================================
// Session Management
// ============================================================================

fn resume_claude_session() -> Result<(), String> {
    // Type "claude --continue" and press Enter
    let script = r#"tell application "System Events"
        keystroke "claude --continue"
        delay 0.1
        key code 36
    end tell"#;
    run_applescript(script)
}

fn stop_claude_session() -> Result<(), String> {
    // Send Ctrl+C to stop current operation
    let script = r#"tell application "System Events"
        keystroke "c" using control down
    end tell"#;
    run_applescript(script)
}

fn start_new_chat() -> Result<(), String> {
    // Type "claude" to start fresh
    let script = r#"tell application "System Events"
        keystroke "claude"
        delay 0.1
        key code 36
    end tell"#;
    run_applescript(script)
}

// ============================================================================
// Monitoring Actions
// ============================================================================

fn show_task_summary(settings: &ClaudeActionSettings) -> Result<(), String> {
    // This would read from the monitor and speak the summary
    let tts = TextToSpeech {
        voice: settings.tts_voice.clone(),
        ..Default::default()
    };

    // TODO: Get actual task summary from monitor
    tts.speak("Task summary: No active tasks")?;
    Ok(())
}

fn show_progress_summary(settings: &ClaudeActionSettings) -> Result<(), String> {
    let tts = TextToSpeech {
        voice: settings.tts_voice.clone(),
        ..Default::default()
    };

    // TODO: Get actual progress from monitor
    tts.speak("Progress: Session idle")?;
    Ok(())
}

// ============================================================================
// Utilities
// ============================================================================

fn normalize_terminal_name(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "iterm" | "iterm2" => "iTerm",
        "terminal" => "Terminal",
        "warp" => "Warp",
        _ => "Terminal",
    }
}

fn run_applescript(script: &str) -> Result<(), String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run osascript: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("AppleScript failed: {stderr}");
        return Err(format!("AppleScript error: {stderr}"));
    }

    Ok(())
}
