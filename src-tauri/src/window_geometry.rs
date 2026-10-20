use crate::json;
use log::warn;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use tauri::{LogicalPosition, LogicalSize, Manager, Monitor, Runtime, WebviewWindow};

#[derive(Deserialize)]
struct SavedWindowGeometry {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

fn load_saved_geometry<R: Runtime>(window: &WebviewWindow<R>) -> Option<SavedWindowGeometry> {
    let path = window
        .path()
        .app_config_dir()
        .ok()?
        .join(tauri_plugin_window_state::DEFAULT_FILENAME);
    let content = fs::read_to_string(path).ok()?;
    let mut geometries: HashMap<String, SavedWindowGeometry> = json::from_str(&content).ok()?;

    geometries.remove(window.label())
}

fn find_saved_monitor<R: Runtime>(
    window: &WebviewWindow<R>,
    geometry: &SavedWindowGeometry,
) -> Option<Monitor> {
    window
        .available_monitors()
        .ok()?
        .into_iter()
        .find(|monitor| {
            let origin = monitor.position();
            let size = monitor.size();

            (origin.x..origin.x + size.width as i32).contains(&geometry.x)
                && (origin.y..origin.y + size.height as i32).contains(&geometry.y)
        })
}

pub fn restore_main_window_geometry<R: Runtime>(window: &WebviewWindow<R>) {
    let Some(geometry) = load_saved_geometry(window) else {
        return;
    };
    let Some(monitor) = find_saved_monitor(window, &geometry) else {
        return;
    };
    let scale = monitor.scale_factor();
    let position =
        LogicalPosition::new(f64::from(geometry.x) / scale, f64::from(geometry.y) / scale);
    let size = LogicalSize::new(
        f64::from(geometry.width) / scale,
        f64::from(geometry.height) / scale,
    );

    if let Err(err) = window
        .set_position(position)
        .and_then(|_| window.set_size(size))
    {
        warn!("[WindowGeometry] failed to restore the main window geometry: {err}");
    }
}
