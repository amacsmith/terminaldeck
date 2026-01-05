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
        "dictate" => toggle_dictation(&settings.dictation_shortcut),
        "submit" => send_keycode(36, None), // Enter
        "up" => send_keycode(126, None),    // Up arrow
        "down" => send_keycode(125, None),  // Down arrow
        "delete" => send_keycode(51, None), // Delete/Backspace

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
    pub dictation_shortcut: String,
}

impl Default for ActionSettings {
    fn default() -> Self {
        ActionSettings {
            terminal_app: "Terminal".to_string(),
            cli_tool: "claude".to_string(),
            dictation_shortcut: "fn_twice".to_string(),
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

/// Toggle dictation using the configured shortcut
fn toggle_dictation(shortcut: &str) -> Result<(), String> {
    let script = match shortcut {
        // Press Fn/Globe key twice (default macOS Dictation on older Macs)
        "fn_twice" => r#"
            tell application "System Events"
                key code 63
                delay 0.15
                key code 63
            end tell
        "#.to_string(),

        // Hold Fn/Globe key (for Wispr Flow or newer macOS)
        "fn_hold" => r#"
            tell application "System Events"
                key down 63
                delay 0.5
                key up 63
            end tell
        "#.to_string(),

        // Press Control key twice (with explicit up/down for reliability)
        "ctrl_twice" => r#"
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

        // Custom shortcut - parse format like "cmd+shift+d"
        custom => {
            if let Some(script) = parse_custom_shortcut(custom) {
                script
            } else {
                // Fall back to ctrl_twice if parsing fails
                r#"
                    tell application "System Events"
                        key code 59
                        delay 0.2
                        key code 59
                    end tell
                "#.to_string()
            }
        }
    };

    run_applescript(&script)?;
    info!("Toggled dictation with shortcut: {}", shortcut);
    Ok(())
}

/// Parse a custom shortcut string like "cmd+shift+d" into AppleScript
fn parse_custom_shortcut(shortcut: &str) -> Option<String> {
    // Parse format like "cmd+shift+d" into AppleScript
    let parts: Vec<String> = shortcut.split('+').map(|s| s.trim().to_lowercase()).collect();

    if parts.is_empty() {
        return None;
    }

    let key = parts.last()?;
    let modifiers: Vec<&str> = parts[..parts.len()-1].iter().map(|s| s.as_str()).collect();

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
