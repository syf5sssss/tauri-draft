use std::sync::{Arc, Mutex};
use tauri::State;

use crate::util::AppConfig;

// 包装配置状态
pub struct AppState {
    pub(crate) config: Arc<Mutex<AppConfig>>,
}

#[tauri::command]
pub fn get_config(state: State<AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

#[tauri::command]
pub fn get_config_field(
    field: String,
    state: State<AppState>,
) -> Result<serde_json::Value, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    let value = serde_json::to_value(&*config).map_err(|e| e.to_string())?;
    value
        .get(&field)
        .cloned()
        .ok_or_else(|| format!("Field '{}' not found", field))
}

#[tauri::command]
pub fn set_config_field(
    app: tauri::AppHandle,
    field: String,
    value: serde_json::Value,
    state: State<AppState>,
) -> Result<(), String> {
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    config.update_field(&field, value)?;
    config.save(&app)?;
    Ok(())
}

#[tauri::command]
pub fn reset_config(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<AppConfig, String> {
    let default_config = AppConfig::default();
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        *config = default_config.clone();
    }
    default_config.save(&app)?;
    Ok(default_config)
}
