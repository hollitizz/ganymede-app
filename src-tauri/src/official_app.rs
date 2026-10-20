use crate::tauri_api_ext::ConfPathExt;
use log::{error, info};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, Runtime};

const OFFICIAL_APP_IDENTIFIER: &str = "com.ganymede.ganymede-app";

fn official_app_config_dir(config_dir: &Path) -> Option<PathBuf> {
    Some(config_dir.parent()?.join(OFFICIAL_APP_IDENTIFIER))
}

fn copy_dir_recursively(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir_recursively(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }

    Ok(())
}

pub fn import_official_app_data<R: Runtime>(app_handle: &AppHandle<R>) {
    let Ok(config_dir) = app_handle.path().app_config_dir() else {
        return;
    };
    let Some(official_config_dir) = official_app_config_dir(&config_dir) else {
        return;
    };

    if app_handle.path().app_conf_file().exists()
        || official_config_dir == config_dir
        || !official_config_dir.is_dir()
    {
        return;
    }

    match copy_dir_recursively(&official_config_dir, &config_dir) {
        Ok(()) => info!("[OfficialApp] Imported data from {:?}", official_config_dir),
        Err(e) => error!("[OfficialApp] Failed to import data: {}", e),
    }
}
