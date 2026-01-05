pub mod actions;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use hidapi::{HidApi, HidDevice};
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

// Embed a font for text rendering (using Inter from Google Fonts, or fallback to system)
const EMBEDDED_FONT: &[u8] = include_bytes!("../../fonts/Inter-Bold.ttf");

// Stream Deck MK.2 constants
const VENDOR_ID: u16 = 0x0fd9;
const PRODUCT_ID_MK2: u16 = 0x0080;
const NUM_KEYS: usize = 15;
#[allow(dead_code)]
const KEY_COLS: usize = 5;
#[allow(dead_code)]
const KEY_ROWS: usize = 3;
const ICON_SIZE: u32 = 72;

// Button configuration
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ButtonConfig {
    pub id: u32,
    pub label: String,
    pub sublabel: Option<String>,
    pub color: String,
    pub action: String,
}

// Get display name for terminal app
fn get_terminal_display_name(terminal_app: &str) -> String {
    match terminal_app.to_lowercase().as_str() {
        "terminal" => "Terminal".to_string(),
        "iterm" | "iterm2" => "iTerm".to_string(),
        "warp" => "Warp".to_string(),
        _ => terminal_app.to_string(),
    }
}

// Get display name for CLI tool
fn get_cli_display_name(cli_tool: &str) -> String {
    match cli_tool.to_lowercase().as_str() {
        "claude" => "Claude".to_string(),
        "codex" => "Codex".to_string(),
        _ => cli_tool.to_string(),
    }
}

// Default button layout (uses default settings)
pub fn get_default_buttons() -> Vec<ButtonConfig> {
    get_buttons_with_settings("Terminal", "claude")
}

// Button layout with custom settings
pub fn get_buttons_with_settings(terminal_app: &str, cli_tool: &str) -> Vec<ButtonConfig> {
    let terminal_name = get_terminal_display_name(terminal_app);
    let cli_name = get_cli_display_name(cli_tool);

    vec![
        // Row 1: Session Management
        // Muted color palette for power users
        ButtonConfig { id: 0, label: "Open".into(), sublabel: Some(terminal_name.clone()), color: "#3d5a80".into(), action: "openTerminal".into() },
        ButtonConfig { id: 1, label: "Switch".into(), sublabel: Some("Window".into()), color: "#2d3748".into(), action: "switchWindow".into() },
        ButtonConfig { id: 2, label: "Launch".into(), sublabel: Some(cli_name), color: "#3d5a80".into(), action: "launchClaude".into() },
        ButtonConfig { id: 3, label: "New".into(), sublabel: Some(terminal_name), color: "#3d5a80".into(), action: "newTerminal".into() },
        ButtonConfig { id: 4, label: "Close".into(), sublabel: Some("Session".into()), color: "#8b3a3a".into(), action: "closeSession".into() },

        // Row 2: Responses
        ButtonConfig { id: 5, label: "Yes".into(), sublabel: Some("[Y]".into()), color: "#3d6b59".into(), action: "yes".into() },
        ButtonConfig { id: 6, label: "Yes to".into(), sublabel: Some("All".into()), color: "#3d6b59".into(), action: "yesToAll".into() },
        ButtonConfig { id: 7, label: "No".into(), sublabel: Some("[N]".into()), color: "#8b3a3a".into(), action: "no".into() },
        ButtonConfig { id: 8, label: "Cancel".into(), sublabel: Some("ESC".into()), color: "#8b3a3a".into(), action: "cancel".into() },
        ButtonConfig { id: 9, label: "Tab".into(), sublabel: Some("->|".into()), color: "#2d3748".into(), action: "tab".into() },

        // Row 3: Primary Actions
        ButtonConfig { id: 10, label: "Dictate".into(), sublabel: Some("MIC".into()), color: "#5a4a78".into(), action: "dictate".into() },
        ButtonConfig { id: 11, label: "Submit".into(), sublabel: Some("ENTER".into()), color: "#3d6b59".into(), action: "submit".into() },
        ButtonConfig { id: 12, label: "UP".into(), sublabel: None, color: "#2d3748".into(), action: "up".into() },
        ButtonConfig { id: 13, label: "DOWN".into(), sublabel: None, color: "#2d3748".into(), action: "down".into() },
        ButtonConfig { id: 14, label: "Delete".into(), sublabel: Some("DEL".into()), color: "#2d3748".into(), action: "delete".into() },
    ]
}

pub struct StreamDeck {
    device: Option<HidDevice>,
    connected: bool,
}

impl StreamDeck {
    pub fn new() -> Self {
        StreamDeck {
            device: None,
            connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        let api = HidApi::new().map_err(|e| format!("Failed to init HID API: {}", e))?;

        // Try to find Stream Deck MK.2
        match api.open(VENDOR_ID, PRODUCT_ID_MK2) {
            Ok(device) => {
                info!("Connected to Stream Deck MK.2");
                self.device = Some(device);
                self.connected = true;
                Ok(())
            }
            Err(e) => {
                warn!("Failed to connect to Stream Deck: {}", e);
                self.connected = false;
                Err(format!("Failed to connect: {}", e))
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected && self.device.is_some()
    }

    pub fn disconnect(&mut self) {
        self.device = None;
        self.connected = false;
        info!("Stream Deck disconnected");
    }

    pub fn set_brightness(&self, percent: u8) -> Result<(), String> {
        let device = self.device.as_ref().ok_or("Not connected")?;

        let brightness = percent.min(100);
        let data = vec![0x03, 0x08, brightness, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        device.send_feature_report(&data)
            .map_err(|e| format!("Failed to set brightness: {}", e))?;

        debug!("Set brightness to {}%", brightness);
        Ok(())
    }

    pub fn reset(&self) -> Result<(), String> {
        let device = self.device.as_ref().ok_or("Not connected")?;

        let data = vec![0x03, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        device.send_feature_report(&data)
            .map_err(|e| format!("Failed to reset: {}", e))?;

        info!("Reset Stream Deck to logo");
        Ok(())
    }

    /// Create a button image with label and optional sublabel
    pub fn create_button_image(label: &str, sublabel: Option<&str>, bg_color: &str) -> Vec<u8> {
        let mut img: RgbaImage = ImageBuffer::new(ICON_SIZE, ICON_SIZE);

        // Parse hex color
        let (r, g, b) = parse_hex_color(bg_color);

        // Fill background
        for pixel in img.pixels_mut() {
            *pixel = Rgba([r, g, b, 255]);
        }

        // Load font for text rendering
        match FontRef::try_from_slice(EMBEDDED_FONT) {
            Ok(font) => {
            let white = Rgba([255u8, 255u8, 255u8, 255u8]);

            // Calculate actual text width using glyph metrics
            let calc_text_width = |text: &str, scale: PxScale, f: &FontRef| -> f32 {
                let scaled_font = f.as_scaled(scale);
                text.chars()
                    .map(|c| scaled_font.h_advance(f.glyph_id(c)))
                    .sum()
            };

            // Calculate font sizes based on text length
            let label_scale = if label.len() <= 2 {
                PxScale::from(28.0)
            } else if label.len() <= 6 {
                PxScale::from(18.0)
            } else {
                PxScale::from(14.0)
            };

            // Calculate text position (center horizontally)
            let label_width = calc_text_width(label, label_scale, &font);
            let label_x = ((ICON_SIZE as f32 - label_width) / 2.0).max(2.0) as i32;

            // If we have a sublabel, position both lines centered vertically
            let label_y = if sublabel.is_some() { 18 } else { 28 };

            // Draw main label
            draw_text_mut(&mut img, white, label_x, label_y, label_scale, &font, label);

            // Draw sublabel if present
            if let Some(sub) = sublabel {
                let sub_scale = if sub.len() <= 2 {
                    PxScale::from(16.0)
                } else {
                    PxScale::from(13.0)
                };
                let sub_width = calc_text_width(sub, sub_scale, &font);
                let sub_x = ((ICON_SIZE as f32 - sub_width) / 2.0).max(2.0) as i32;
                draw_text_mut(&mut img, white, sub_x, 42, sub_scale, &font, sub);
            }
            }
            Err(e) => {
                warn!("Failed to load font for button rendering: {:?}", e);
            }
        }

        // Rotate 180° - Stream Deck MK.2 raw HID protocol displays images upside down
        let rotated = image::DynamicImage::ImageRgba8(img).rotate180();

        // Convert RGBA to RGB (JPEG doesn't support alpha channel)
        let rgb_img: image::RgbImage = rotated.into_rgb8();

        // Convert to JPEG format for Stream Deck
        let mut jpeg_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut jpeg_data);
        if let Err(e) = rgb_img.write_to(&mut cursor, image::ImageFormat::Jpeg) {
            warn!("Failed to encode JPEG: {:?}", e);
        }

        jpeg_data
    }

    pub fn set_key_image(&self, key_index: usize, image_data: &[u8]) -> Result<(), String> {
        let device = self.device.as_ref().ok_or("Not connected")?;

        if key_index >= NUM_KEYS {
            return Err(format!("Invalid key index: {}", key_index));
        }

        // Stream Deck MK.2 uses a specific protocol for setting images
        // Image data is sent in chunks with headers
        let page_size = 1024 - 8; // Max payload per page
        let mut page_number = 0;
        let mut bytes_remaining = image_data.len();
        let mut offset = 0;

        while bytes_remaining > 0 {
            let chunk_size = bytes_remaining.min(page_size);
            let is_last = bytes_remaining <= page_size;

            let mut header = vec![
                0x02, 0x07, key_index as u8,
                if is_last { 1 } else { 0 },
                (chunk_size & 0xff) as u8,
                ((chunk_size >> 8) & 0xff) as u8,
                (page_number & 0xff) as u8,
                ((page_number >> 8) & 0xff) as u8,
            ];

            header.extend_from_slice(&image_data[offset..offset + chunk_size]);

            // Pad to 1024 bytes
            header.resize(1024, 0);

            device.write(&header)
                .map_err(|e| format!("Failed to write image: {}", e))?;

            bytes_remaining -= chunk_size;
            offset += chunk_size;
            page_number += 1;
        }

        Ok(())
    }

    /// Read button presses (blocking with timeout)
    #[allow(dead_code)]
    pub fn read_keys(&self, timeout_ms: i32) -> Result<Option<Vec<bool>>, String> {
        let device = self.device.as_ref().ok_or("Not connected")?;

        let mut buf = [0u8; 512];

        match device.read_timeout(&mut buf, timeout_ms) {
            Ok(size) if size > 0 => {
                // MK.2 button state format: [0x01, 0x00, 0x0F, 0x00, key1, key2, ..., key15]
                // Bytes 0-3 are header, button states start at byte 4
                if buf[0] == 0x01 && size >= 19 {
                    let states: Vec<bool> = (0..NUM_KEYS)
                        .map(|i| buf[i + 4] == 1)
                        .collect();
                    Ok(Some(states))
                } else {
                    log::trace!("Unexpected HID data: header={:#x}, size={}", buf[0], size);
                    Ok(None)
                }
            }
            Ok(_) => Ok(None), // Timeout, no data
            Err(e) => Err(format!("Read error: {}", e)),
        }
    }
}

fn parse_hex_color(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    } else {
        (0, 0, 0)
    }
}

// Stream Deck manager that runs in background
pub struct StreamDeckManager {
    deck: Arc<Mutex<StreamDeck>>,
    #[allow(dead_code)]
    button_sender: Option<mpsc::UnboundedSender<usize>>,
}

impl StreamDeckManager {
    pub fn new() -> Self {
        StreamDeckManager {
            deck: Arc::new(Mutex::new(StreamDeck::new())),
            button_sender: None,
        }
    }

    pub fn get_deck(&self) -> Arc<Mutex<StreamDeck>> {
        Arc::clone(&self.deck)
    }

    pub fn connect(&self) -> Result<(), String> {
        let mut deck = self.deck.lock().map_err(|_| "Lock error")?;
        deck.connect()
    }

    pub fn is_connected(&self) -> bool {
        self.deck.lock().map(|d| d.is_connected()).unwrap_or(false)
    }

    pub fn disconnect(&self) {
        if let Ok(mut deck) = self.deck.lock() {
            deck.disconnect();
        }
    }

    /// Reconnect to Stream Deck (disconnect first, then connect)
    pub fn reconnect(&self) -> Result<(), String> {
        self.disconnect();
        // Small delay to allow device to reset
        thread::sleep(Duration::from_millis(500));
        self.connect()
    }

    /// Start background thread to monitor button presses
    #[allow(dead_code)]
    pub fn start_monitoring(&mut self) -> mpsc::UnboundedReceiver<usize> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.button_sender = Some(tx.clone());

        let deck = Arc::clone(&self.deck);

        thread::spawn(move || {
            let mut last_states = vec![false; NUM_KEYS];

            loop {
                if let Ok(deck_guard) = deck.lock() {
                    if deck_guard.is_connected() {
                        if let Ok(Some(states)) = deck_guard.read_keys(100) {
                            // Detect button press (transition from false to true)
                            for (i, (&current, &last)) in states.iter().zip(last_states.iter()).enumerate() {
                                if current && !last {
                                    debug!("Button {} pressed", i);
                                    let _ = tx.send(i);
                                }
                            }
                            last_states = states;
                        }
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        rx
    }

    /// Update all button images
    pub fn update_buttons(&self, buttons: &[ButtonConfig]) -> Result<(), String> {
        let deck = self.deck.lock().map_err(|_| "Lock error")?;

        for button in buttons {
            let image = StreamDeck::create_button_image(
                &button.label,
                button.sublabel.as_deref(),
                &button.color,
            );
            deck.set_key_image(button.id as usize, &image)?;
        }

        Ok(())
    }
}
