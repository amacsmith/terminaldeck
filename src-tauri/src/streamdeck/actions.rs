use log::{debug, error, info};
use std::process::Command;

/// Execute an AppleScript
fn run_applescript(script: &str) -> Result<(), String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run osascript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("AppleScript failed: {}", stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

/// Send keystrokes to the frontmost application
fn send_keystroke(key: &str, modifiers: Option<&str>) -> Result<(), String> {
    let script = if let Some(mods) = modifiers {
        format!(
            r#"tell application "System Events" to keystroke "{}" using {}"#,
            key, mods
        )
    } else {
        format!(
            r#"tell application "System Events" to keystroke "{}""#,
            key
        )
    };
    run_applescript(&script)
}

/// Send a key code (for special keys like arrow keys, escape, etc.)
fn send_keycode(code: u8, modifiers: Option<&str>) -> Result<(), String> {
    let script = if let Some(mods) = modifiers {
        format!(
            r#"tell application "System Events" to key code {} using {}"#,
            code, mods
        )
    } else {
        format!(
            r#"tell application "System Events" to key code {}"#,
            code
        )
    };
    run_applescript(&script)
}

/// Execute a button action
pub fn execute_action(action: &str, settings: &ActionSettings) -> Result<(), String> {
    info!("Executing action: {}", action);

    match action {
        // Session Management
        "openTerminal" => open_terminal(&settings.terminal_app),
        "newTerminal" => new_terminal(&settings.terminal_app),
        "switchWindow" => switch_window(),
        "launchClaude" => launch_cli(&settings.cli_tool),
        "closeSession" => close_session(),

        // Responses
        "yes" => {
            send_keystroke("y", None)?;
            send_keycode(36, None) // Enter
        }
        "yesToAll" => {
            send_keystroke("a", None)?;
            send_keycode(36, None) // Enter
        }
        "no" => {
            send_keystroke("n", None)?;
            send_keycode(36, None) // Enter
        }
        "cancel" => send_keycode(53, None), // Escape
        "tab" => send_keycode(48, None),    // Tab

        // Primary Actions
        "dictate" => toggle_dictation(),
        "submit" => send_keycode(36, None), // Enter
        "up" => send_keycode(126, None),    // Up arrow
        "down" => send_keycode(125, None),  // Down arrow
        "compact" => {
            send_keystroke("/compact", None)?;
            send_keycode(36, None) // Enter
        }

        _ => {
            error!("Unknown action: {}", action);
            Err(format!("Unknown action: {}", action))
        }
    }
}

/// Settings for action execution
#[derive(Debug, Clone)]
pub struct ActionSettings {
    pub terminal_app: String,
    pub cli_tool: String,
}

impl Default for ActionSettings {
    fn default() -> Self {
        ActionSettings {
            terminal_app: "Terminal".to_string(),
            cli_tool: "claude".to_string(),
        }
    }
}

/// Open/focus the terminal application (without creating new windows)
fn open_terminal(app: &str) -> Result<(), String> {
    let app_name = match app {
        "iterm" | "iTerm" | "iTerm2" => "iTerm",
        "terminal" | "Terminal" => "Terminal",
        "warp" | "Warp" => "Warp",
        custom => custom,
    };

    // For iTerm, use System Events to bring to front without creating new windows
    // Direct "activate" can trigger iTerm's new window creation behavior
    if app_name == "iTerm" {
        let script = r#"
            tell application "System Events"
                if (exists process "iTerm2") then
                    set frontmost of process "iTerm2" to true
                else
                    tell application "iTerm" to activate
                end if
            end tell
        "#;
        run_applescript(script)?;
        info!("Opened iTerm");
        return Ok(());
    }

    // For other apps, use 'open -a' which brings app to front
    let output = Command::new("open")
        .arg("-a")
        .arg(app_name)
        .output()
        .map_err(|e| format!("Failed to run open command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to open {}: {}", app_name, stderr));
    }

    info!("Opened {}", app_name);
    Ok(())
}

/// Create a new terminal window
fn new_terminal(app: &str) -> Result<(), String> {
    let script = match app {
        "terminal" | "Terminal" => r#"
            tell application "Terminal"
                do script ""
                activate
            end tell
        "#.to_string(),
        "iterm" | "iTerm" | "iTerm2" => r#"
            tell application "iTerm"
                create window with default profile
                activate
            end tell
        "#.to_string(),
        "warp" | "Warp" => r#"
            tell application "Warp"
                activate
            end tell
            tell application "System Events"
                keystroke "n" using command down
            end tell
        "#.to_string(),
        custom => format!(
            r#"tell application "{}" to activate"#,
            custom
        ),
    };

    run_applescript(&script)?;
    info!("Created new terminal window");
    Ok(())
}

/// Switch between windows of the frontmost application
fn switch_window() -> Result<(), String> {
    // Use accessibility API to raise the next window
    let script = r#"
        tell application "System Events"
            set frontApp to first application process whose frontmost is true
            set allWindows to windows of frontApp
            set windowCount to count of allWindows
            if windowCount > 1 then
                -- Raise the second window to bring it to front
                perform action "AXRaise" of window 2 of frontApp
            end if
        end tell
    "#;
    run_applescript(script)?;
    debug!("Switched window");
    Ok(())
}

/// Launch the CLI tool
fn launch_cli(cli: &str) -> Result<(), String> {
    let cmd = match cli {
        "claude" => "claude",
        "codex" => "codex",
        custom => custom,
    };

    // Type the command and press Enter
    send_keystroke(cmd, None)?;
    send_keycode(36, None)?; // Enter

    info!("Launched {}", cmd);
    Ok(())
}

/// Close the current session (Ctrl+C, then exit)
fn close_session() -> Result<(), String> {
    // Send Ctrl+C
    send_keystroke("c", Some("control down"))?;

    // Wait a bit
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Type "exit" and press Enter
    send_keystroke("exit", None)?;
    send_keycode(36, None)?;

    info!("Closed session");
    Ok(())
}

/// Toggle macOS dictation
fn toggle_dictation() -> Result<(), String> {
    // Press Fn key twice to trigger dictation
    // Requires: System Settings → Keyboard → Dictation → Shortcut: "Press Fn Key Twice"
    let script = r#"
        tell application "System Events"
            key code 63
            delay 0.15
            key code 63
        end tell
    "#;
    run_applescript(script)?;
    info!("Toggled dictation");
    Ok(())
}
