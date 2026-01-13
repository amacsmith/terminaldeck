//! Tauri commands for Claude Code integration.

use crate::claude::{
    actions::{execute_claude_action, ClaudeActionSettings},
    monitor::{LogEntry, ProgressMonitor, SessionInfo, SessionPhase, TaskState},
    speech::{SpeechToText, TextToSpeech},
    ClaudeCodeManager,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Mutex;
use tauri::State;

// ============================================================================
// State Management
// ============================================================================

/// Global Claude Code manager state
pub struct ClaudeState(pub Mutex<ClaudeCodeManager>);

/// Global progress monitor state
pub struct MonitorState(pub Mutex<ProgressMonitor>);

/// Global TTS settings state
pub struct TtsState(pub Mutex<TtsSettings>);

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct TtsSettings {
    pub enabled: bool,
    pub voice: String,
    pub rate: u16,
    pub last_text: String,
}

// ============================================================================
// Claude State Commands
// ============================================================================

/// Get current Claude Code state
#[tauri::command]
#[specta::specta]
pub fn claude_get_state(state: State<'_, ClaudeState>) -> crate::claude::cli::ClaudeState {
    let manager = state.0.lock().unwrap();
    manager.get_state()
}

/// Check if Claude Code is running
#[tauri::command]
#[specta::specta]
pub fn claude_is_running(state: State<'_, ClaudeState>) -> bool {
    let manager = state.0.lock().unwrap();
    manager.is_running()
}

// ============================================================================
// Speech Commands
// ============================================================================

/// Speak text using TTS
#[tauri::command]
#[specta::specta]
pub fn claude_speak(text: String, tts_state: State<'_, TtsState>) -> Result<(), String> {
    let mut settings = tts_state.0.lock().map_err(|_| "Lock error")?;

    let tts = TextToSpeech {
        voice: settings.voice.clone(),
        rate: settings.rate,
        enabled: settings.enabled,
    };

    tts.speak(&text)?;
    settings.last_text = text;
    Ok(())
}

/// Speak a summary of the given text
#[tauri::command]
#[specta::specta]
pub fn claude_speak_summary(
    text: String,
    claude_state: State<'_, ClaudeState>,
    tts_state: State<'_, TtsState>,
) -> Result<(), String> {
    let manager = claude_state.0.lock().map_err(|_| "Lock error")?;
    let summary = manager.summarize_for_tts(&text);

    drop(manager); // Release lock before speaking

    let mut settings = tts_state.0.lock().map_err(|_| "Lock error")?;
    let tts = TextToSpeech {
        voice: settings.voice.clone(),
        rate: settings.rate,
        enabled: settings.enabled,
    };

    tts.speak(&summary)?;
    settings.last_text = summary;
    Ok(())
}

/// Toggle TTS enabled state
#[tauri::command]
#[specta::specta]
pub fn claude_toggle_tts(tts_state: State<'_, TtsState>) -> bool {
    let mut settings = tts_state.0.lock().unwrap();
    settings.enabled = !settings.enabled;
    TextToSpeech::set_enabled(settings.enabled);
    info!("TTS toggled: {}", settings.enabled);
    settings.enabled
}

/// Get TTS settings
#[tauri::command]
#[specta::specta]
pub fn claude_get_tts_settings(tts_state: State<'_, TtsState>) -> TtsSettings {
    tts_state.0.lock().unwrap().clone()
}

/// Set TTS settings
#[tauri::command]
#[specta::specta]
pub fn claude_set_tts_settings(settings: TtsSettings, tts_state: State<'_, TtsState>) {
    let mut state = tts_state.0.lock().unwrap();
    *state = settings;
    TextToSpeech::set_enabled(state.enabled);
}

/// Activate speech-to-text
#[tauri::command]
#[specta::specta]
pub fn claude_activate_stt() -> Result<(), String> {
    let stt = SpeechToText::default();
    stt.activate()
}

/// Deactivate speech-to-text
#[tauri::command]
#[specta::specta]
pub fn claude_deactivate_stt() -> Result<(), String> {
    let stt = SpeechToText::default();
    stt.deactivate()
}

/// Toggle speech-to-text
#[tauri::command]
#[specta::specta]
pub fn claude_toggle_stt() -> Result<bool, String> {
    let stt = SpeechToText::default();
    let is_active = SpeechToText::is_active();

    if is_active {
        stt.deactivate()?;
    } else {
        stt.activate()?;
    }

    Ok(!is_active)
}

// ============================================================================
// Monitor Commands
// ============================================================================

/// Start a new monitoring session
#[tauri::command]
#[specta::specta]
pub fn claude_start_session(name: Option<String>, monitor_state: State<'_, MonitorState>) {
    let mut monitor = monitor_state.0.lock().unwrap();
    monitor.start_session(name);
}

/// Get current session info
#[tauri::command]
#[specta::specta]
pub fn claude_get_session(monitor_state: State<'_, MonitorState>) -> SessionInfo {
    let monitor = monitor_state.0.lock().unwrap();
    monitor.get_session()
}

/// Set session phase
#[tauri::command]
#[specta::specta]
pub fn claude_set_phase(phase: String, monitor_state: State<'_, MonitorState>) {
    let monitor = monitor_state.0.lock().unwrap();
    monitor.set_phase(SessionPhase::from_str(&phase));
}

/// Add a task to monitor
#[tauri::command]
#[specta::specta]
pub fn claude_add_task(description: String, monitor_state: State<'_, MonitorState>) -> u32 {
    let monitor = monitor_state.0.lock().unwrap();
    monitor.add_task(description)
}

/// Update task status
#[tauri::command]
#[specta::specta]
pub fn claude_update_task(task_id: u32, status: String, monitor_state: State<'_, MonitorState>) {
    let monitor = monitor_state.0.lock().unwrap();
    let state = match status.as_str() {
        "pending" => TaskState::Pending,
        "in_progress" => TaskState::InProgress,
        "completed" => TaskState::Completed,
        "skipped" => TaskState::Skipped,
        "failed" => TaskState::Failed,
        _ => TaskState::Pending,
    };
    monitor.update_task(task_id, state);
}

/// Set next steps
#[tauri::command]
#[specta::specta]
pub fn claude_set_next_steps(steps: Vec<String>, monitor_state: State<'_, MonitorState>) {
    let monitor = monitor_state.0.lock().unwrap();
    monitor.set_next_steps(steps);
}

/// Get recent logs
#[tauri::command]
#[specta::specta]
pub fn claude_get_logs(limit: usize, monitor_state: State<'_, MonitorState>) -> Vec<LogEntry> {
    let monitor = monitor_state.0.lock().unwrap();
    monitor.get_logs(limit)
}

/// Process output line (for parsing Claude Code output)
#[tauri::command]
#[specta::specta]
pub fn claude_process_output(line: String, monitor_state: State<'_, MonitorState>) {
    let monitor = monitor_state.0.lock().unwrap();
    monitor.process_output(&line);
}

// ============================================================================
// Action Commands
// ============================================================================

/// Execute a Claude-specific action
#[tauri::command]
#[specta::specta]
pub fn claude_execute_action(
    action: String,
    terminal_app: String,
    tts_state: State<'_, TtsState>,
) -> Result<(), String> {
    let settings = tts_state.0.lock().map_err(|_| "Lock error")?;

    let action_settings = ClaudeActionSettings {
        terminal_app,
        tts_enabled: settings.enabled,
        tts_voice: settings.voice.clone(),
        stt_method: "ctrl_twice".to_string(),
        last_tts_text: settings.last_text.clone(),
    };

    drop(settings); // Release lock

    execute_claude_action(&action, &action_settings)
}

/// Send a numbered response (1-4)
#[tauri::command]
#[specta::specta]
pub fn claude_respond_number(number: u8) -> Result<(), String> {
    if number < 1 || number > 9 {
        return Err("Number must be between 1 and 9".to_string());
    }

    use std::process::Command;
    let script = format!(
        r#"tell application "System Events"
            keystroke "{number}"
            delay 0.1
            key code 36
        end tell"#
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to send number: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {stderr}"));
    }

    Ok(())
}

// ============================================================================
// Terminal Switching Commands
// ============================================================================

/// Switch to next terminal window
#[tauri::command]
#[specta::specta]
pub fn claude_next_terminal(terminal_app: String) -> Result<(), String> {
    use std::process::Command;

    let app = match terminal_app.to_lowercase().as_str() {
        "iterm" | "iterm2" => "iTerm",
        "warp" => "Warp",
        _ => "Terminal",
    };

    let script = format!(
        r#"tell application "{app}"
            activate
        end tell
        tell application "System Events"
            keystroke "`" using command down
        end tell"#
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to switch terminal: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {stderr}"));
    }

    Ok(())
}

/// Switch to specific terminal tab
#[tauri::command]
#[specta::specta]
pub fn claude_terminal_tab(tab_number: u8, terminal_app: String) -> Result<(), String> {
    use std::process::Command;

    let app = match terminal_app.to_lowercase().as_str() {
        "iterm" | "iterm2" => "iTerm",
        "warp" => "Warp",
        _ => "Terminal",
    };

    let script = format!(
        r#"tell application "{app}"
            activate
        end tell
        tell application "System Events"
            keystroke "{tab_number}" using command down
        end tell"#
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to switch tab: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {stderr}"));
    }

    Ok(())
}

// ============================================================================
// Project Commands
// ============================================================================

/// Send a project command to Claude Code
#[tauri::command]
#[specta::specta]
pub fn claude_project_command(command: String) -> Result<(), String> {
    use std::process::Command as SysCommand;

    let cmd = match command.as_str() {
        "build" => "/build",
        "test" => "/test",
        "commit" => "/commit",
        "pr" => "/pr",
        "help" => "/help",
        _ => &command,
    };

    let script = format!(
        r#"tell application "System Events"
            keystroke "{cmd}"
            delay 0.1
            key code 36
        end tell"#
    );

    let output = SysCommand::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to send command: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {stderr}"));
    }

    info!("Sent project command: {cmd}");
    Ok(())
}
