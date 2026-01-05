use crate::streamdeck::{
    actions::{execute_action, ActionSettings},
    get_default_buttons, get_buttons_with_settings, ButtonConfig, StreamDeckManager,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::State;

// Global flag to control button monitoring thread
static MONITORING_ACTIVE: AtomicBool = AtomicBool::new(false);

// Global settings for button actions
static TERMINAL_APP: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());
static CLI_TOOL: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());
static DICTATION_SHORTCUT: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());

fn get_action_settings() -> ActionSettings {
    let terminal = TERMINAL_APP.read().unwrap();
    let cli = CLI_TOOL.read().unwrap();
    let dictation = DICTATION_SHORTCUT.read().unwrap();
    ActionSettings {
        terminal_app: if terminal.is_empty() { "Terminal".to_string() } else { terminal.clone() },
        cli_tool: if cli.is_empty() { "claude".to_string() } else { cli.clone() },
        dictation_shortcut: if dictation.is_empty() { "ctrl_twice".to_string() } else { dictation.clone() },
    }
}

fn set_action_settings(terminal_app: &str, cli_tool: &str, dictation_shortcut: &str) {
    *TERMINAL_APP.write().unwrap() = terminal_app.to_string();
    *CLI_TOOL.write().unwrap() = cli_tool.to_string();
    *DICTATION_SHORTCUT.write().unwrap() = dictation_shortcut.to_string();
}

/// Global Stream Deck manager state
pub struct DeckState(pub Mutex<StreamDeckManager>);

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DeckStatus {
    pub connected: bool,
    pub buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct DeckSettings {
    pub terminal_app: String,
    pub cli_tool: String,
    pub dictation_shortcut: String,
}

impl Default for DeckSettings {
    fn default() -> Self {
        DeckSettings {
            terminal_app: "Terminal".to_string(),
            cli_tool: "claude".to_string(),
            dictation_shortcut: "ctrl_twice".to_string(),
        }
    }
}

/// Check Stream Deck connection status
#[tauri::command]
#[specta::specta]
pub fn deck_status(state: State<'_, DeckState>) -> DeckStatus {
    let manager = state.0.lock().unwrap();
    let is_connected = manager.is_connected();

    // If was connected but now disconnected, stop monitoring
    if !is_connected && MONITORING_ACTIVE.load(Ordering::SeqCst) {
        MONITORING_ACTIVE.store(false, Ordering::SeqCst);
        debug!("Stream Deck disconnected - stopped monitoring");
    }

    DeckStatus {
        connected: is_connected,
        buttons: get_default_buttons(),
    }
}

/// Connect to Stream Deck
#[tauri::command]
#[specta::specta]
pub async fn deck_connect(state: State<'_, DeckState>) -> Result<bool, String> {
    // Clone the Arc to move into the blocking task
    let deck_arc = {
        let manager = state.0.lock().map_err(|_| "Lock error")?;
        manager.get_deck()
    };

    // Check if already connected
    let already_connected = {
        let deck = deck_arc.lock().map_err(|_| "Lock error")?;
        deck.is_connected()
    };

    // Get buttons config before the blocking task
    let buttons = get_default_buttons();
    let buttons_for_monitor = buttons.clone();

    // Only connect if not already connected
    if !already_connected {
        let deck_arc_clone = Arc::clone(&deck_arc);
        tauri::async_runtime::spawn_blocking(move || {
            let mut deck = deck_arc_clone.lock().map_err(|_| "Lock error".to_string())?;
            deck.connect()?;
            deck.set_brightness(80)?;

            // Update button images
            for button in &buttons {
                let image = crate::streamdeck::StreamDeck::create_button_image(
                    &button.label,
                    button.sublabel.as_deref(),
                    &button.color,
                );
                deck.set_key_image(button.id as usize, &image)?;
            }

            Ok::<(), String>(())
        })
        .await
        .map_err(|e| format!("Task error: {}", e))??;
    } else {
        debug!("Stream Deck already connected, starting monitoring only");
    }

    // Start button monitoring thread if not already running
    if !MONITORING_ACTIVE.load(Ordering::SeqCst) {
        MONITORING_ACTIVE.store(true, Ordering::SeqCst);
        let deck_for_monitor = Arc::clone(&deck_arc);

        thread::spawn(move || {
            let mut last_states = vec![false; 15];
            let mut hold_start: [Option<std::time::Instant>; 15] = Default::default();
            let mut last_repeat: [Option<std::time::Instant>; 15] = Default::default();

            // Hold-to-repeat settings
            let initial_delay = Duration::from_millis(400); // Delay before repeat starts
            let repeat_interval = Duration::from_millis(50); // Interval between repeats

            while MONITORING_ACTIVE.load(Ordering::SeqCst) {
                if let Ok(deck) = deck_for_monitor.lock() {
                    if deck.is_connected() {
                        if let Ok(Some(states)) = deck.read_keys(100) {
                            for (i, (&current, &last)) in states.iter().zip(last_states.iter()).enumerate() {
                                // Device index maps directly to button ID (no offset)
                                let button_id = i;

                                if current && !last {
                                    // Button just pressed - execute action immediately
                                    let action_settings = get_action_settings();
                                    if let Some(button) = buttons_for_monitor.iter().find(|b| b.id as usize == button_id) {
                                        debug!("Button {} ({}) pressed", button_id, button.label);
                                        let _ = execute_action(&button.action, &action_settings);
                                    }
                                    // Start hold timer
                                    hold_start[i] = Some(std::time::Instant::now());
                                    last_repeat[i] = None;
                                } else if current && last {
                                    // Button being held - check for repeat (delete button only)
                                    if let Some(button) = buttons_for_monitor.iter().find(|b| b.id as usize == button_id) {
                                        if button.action == "delete" {
                                            if let Some(start) = hold_start[i] {
                                                let held_for = start.elapsed();
                                                if held_for >= initial_delay {
                                                    // Past initial delay, check repeat interval
                                                    let should_repeat = match last_repeat[i] {
                                                        None => true,
                                                        Some(last) => last.elapsed() >= repeat_interval,
                                                    };
                                                    if should_repeat {
                                                        let action_settings = get_action_settings();
                                                        let _ = execute_action(&button.action, &action_settings);
                                                        last_repeat[i] = Some(std::time::Instant::now());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if !current && last {
                                    // Button released - reset hold timer
                                    hold_start[i] = None;
                                    last_repeat[i] = None;
                                }
                            }
                            last_states = states;
                        }
                    }
                }
                thread::sleep(Duration::from_millis(30)); // Faster polling for smoother repeat
            }
            debug!("Button monitoring stopped");
        });

        info!("Button monitoring started");
    }

    info!("Stream Deck connected and initialized");
    Ok(true)
}

/// Disconnect from Stream Deck
#[tauri::command]
#[specta::specta]
pub fn deck_disconnect(state: State<'_, DeckState>) -> Result<(), String> {
    // Stop monitoring thread
    MONITORING_ACTIVE.store(false, Ordering::SeqCst);

    let manager = state.0.lock().map_err(|_| "Lock error")?;

    // Reset Stream Deck to logo before disconnecting
    let deck_arc = manager.get_deck();
    if let Ok(deck_guard) = deck_arc.lock() {
        if deck_guard.is_connected() {
            let _ = deck_guard.reset();
        }
    }
    drop(deck_arc);

    manager.disconnect();
    info!("Stream Deck disconnected");
    Ok(())
}

/// Reconnect to Stream Deck (useful after unplug/replug)
#[tauri::command]
#[specta::specta]
pub fn deck_reconnect(state: State<'_, DeckState>) -> Result<bool, String> {
    let manager = state.0.lock().map_err(|_| "Lock error")?;
    manager.reconnect()?;

    // Set brightness and update button images
    {
        let deck = manager.get_deck();
        let deck_guard = deck.lock().map_err(|_| "Lock error")?;
        deck_guard.set_brightness(80)?;
    }

    // Update button images
    let buttons = get_default_buttons();
    manager.update_buttons(&buttons)?;

    info!("Stream Deck reconnected and initialized");
    Ok(true)
}

/// Execute a button action
#[tauri::command]
#[specta::specta]
pub fn deck_execute_action(
    action: String,
    settings: DeckSettings,
) -> Result<(), String> {
    let action_settings = ActionSettings {
        terminal_app: settings.terminal_app,
        cli_tool: settings.cli_tool,
        dictation_shortcut: settings.dictation_shortcut,
    };

    execute_action(&action, &action_settings)
}

/// Get the button configuration
#[tauri::command]
#[specta::specta]
pub fn deck_get_buttons() -> Vec<ButtonConfig> {
    get_default_buttons()
}

/// Get button configuration with custom settings
#[tauri::command]
#[specta::specta]
pub fn deck_get_buttons_with_settings(settings: DeckSettings) -> Vec<ButtonConfig> {
    get_buttons_with_settings(&settings.terminal_app, &settings.cli_tool)
}

/// Update Stream Deck button images
#[tauri::command]
#[specta::specta]
pub fn deck_update_buttons(state: State<'_, DeckState>) -> Result<(), String> {
    let manager = state.0.lock().map_err(|_| "Lock error")?;
    let buttons = get_default_buttons();
    manager.update_buttons(&buttons)
}

/// Set action settings for button monitoring (without updating button images)
#[tauri::command]
#[specta::specta]
pub fn deck_set_action_settings(settings: DeckSettings) {
    set_action_settings(&settings.terminal_app, &settings.cli_tool, &settings.dictation_shortcut);
    info!("Set action settings: terminal={}, cli={}, dictation={}", settings.terminal_app, settings.cli_tool, settings.dictation_shortcut);
}

/// Update Stream Deck button images with custom settings
#[tauri::command]
#[specta::specta]
pub async fn deck_update_buttons_with_settings(
    state: State<'_, DeckState>,
    settings: DeckSettings,
) -> Result<(), String> {
    // Update global action settings for button monitoring
    set_action_settings(&settings.terminal_app, &settings.cli_tool, &settings.dictation_shortcut);

    // Get the deck Arc
    let deck_arc = {
        let manager = state.0.lock().map_err(|_| "Lock error")?;
        manager.get_deck()
    };

    let buttons = get_buttons_with_settings(&settings.terminal_app, &settings.cli_tool);

    // Run blocking HID operations in a separate thread
    tauri::async_runtime::spawn_blocking(move || {
        let deck = deck_arc.lock().map_err(|_| "Lock error".to_string())?;
        if !deck.is_connected() {
            return Err("Stream Deck not connected".to_string());
        }

        for button in &buttons {
            let image = crate::streamdeck::StreamDeck::create_button_image(
                &button.label,
                button.sublabel.as_deref(),
                &button.color,
            );
            deck.set_key_image(button.id as usize, &image)?;
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;

    info!("Updated Stream Deck buttons with settings: {:?}", settings);
    Ok(())
}

/// Debug: Save a test button image to disk
#[tauri::command]
#[specta::specta]
pub fn deck_debug_save_image() -> Result<String, String> {
    use crate::streamdeck::StreamDeck;
    use std::fs;

    let image_data = StreamDeck::create_button_image("Open", Some("Terminal"), "#2563eb");
    let path = "/tmp/streamdeck_test_button.jpg";
    fs::write(path, &image_data).map_err(|e| format!("Failed to write: {}", e))?;
    info!("Saved test button image to {}", path);
    Ok(path.to_string())
}
