//! Recording indicator window management

use crate::error::Result;
use log::info;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const INDICATOR_LABEL: &str = "recording";

#[derive(Clone, Serialize)]
pub struct AudioLevel {
    pub level: f32,
    pub peak: f32,
}

/// Show the recording indicator window
pub fn show_indicator(handle: &AppHandle) -> Result<()> {
    // Try to get existing window or create new one
    if let Some(window) = handle.get_webview_window(INDICATOR_LABEL) {
        // Navigate to the recording route and show
        let _ = window.eval("window.location.href = '/recording'");
        let _ = window.show();
        let _ = window.set_focus();
        info!("Recording indicator shown");
    } else {
        // Create the window if it doesn't exist
        let window = WebviewWindowBuilder::new(
            handle,
            INDICATOR_LABEL,
            WebviewUrl::App("/recording".into()),
        )
        .title("")
        .inner_size(200.0, 60.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .visible(true)
        .build()?;

        // Position near top-center of screen
        if let Ok(monitor) = window.current_monitor() {
            if let Some(monitor) = monitor {
                let size = monitor.size();
                let x = (size.width as i32 - 200) / 2;
                let y = 50;
                let _ = window.set_position(tauri::Position::Physical(
                    tauri::PhysicalPosition::new(x, y),
                ));
            }
        }

        info!("Recording indicator window created");
    }

    Ok(())
}

/// Hide the recording indicator window
pub fn hide_indicator(handle: &AppHandle) -> Result<()> {
    if let Some(window) = handle.get_webview_window(INDICATOR_LABEL) {
        let _ = window.hide();
        info!("Recording indicator hidden");
    }
    Ok(())
}

/// Emit an audio level update to the indicator
pub fn emit_audio_level(handle: &AppHandle, level: f32, peak: f32) {
    let _ = handle.emit_to(
        INDICATOR_LABEL,
        "audio-level",
        AudioLevel { level, peak },
    );
}

/// Emit processing state to the indicator
pub fn emit_processing(handle: &AppHandle, processing: bool) {
    let _ = handle.emit_to(INDICATOR_LABEL, "recording-processing", processing);
}
