use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub username: String,
    pub password: String,
    pub ip: String,
    pub isdebug: bool,
    pub volume: f64,
    pub notifications: Vec<String>,
    pub timeout: u32,
    pub alarms: Vec<AlarmList>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AlarmList {
    pub index: u32,
    pub name: String,
    pub enname: String,
    pub level: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            username: "sa".to_string(),
            password: "Nanhui-380".to_string(),
            ip: "127.0.0.1".to_string(),
            isdebug: false,
            volume: 0.8,
            notifications: vec!["update".to_string(), "message".to_string()],
            timeout: 30,
            alarms: vec![
                AlarmList {
                    index: 1,
                    name: "识别失败".to_string(),
                    enname: "identification failed".to_string(),
                    level: 0,
                },
                AlarmList {
                    index: 2,
                    name: "通讯失败".to_string(),
                    enname: "communication failure".to_string(),
                    level: 1,
                },
            ],
        }
    }
}

impl AppConfig {
    // 获取配置文件路径 - 现在需要 AppHandle 参数
    pub fn config_path(app_handle: &tauri::AppHandle) -> PathBuf {
        app_handle
            .path()
            .resolve("config.json", BaseDirectory::AppConfig)
            .expect("failed to resolve config path")
    }

    // 加载或创建配置 - 现在需要 AppHandle 参数
    pub fn load(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let config_path = Self::config_path(app_handle);

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save(app_handle)?; // 保存时需要传递 app_handle
            return Ok(default_config);
        }

        let config_str = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        serde_json::from_str(&config_str).map_err(|e| format!("Failed to parse config: {}", e))
    }

    // 保存配置 - 现在需要 AppHandle 参数
    pub fn save(&self, app_handle: &tauri::AppHandle) -> Result<(), String> {
        let config_path = Self::config_path(app_handle);

        let parent = config_path.parent().expect("failed to get parent dir");

        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {}", e))?;

        let config_str = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, config_str).map_err(|e| format!("Failed to write config: {}", e))
    }

    // 修复所有权问题
    pub fn update_field<T: serde::Serialize>(
        &mut self,
        field: &str,
        value: T,
    ) -> Result<(), String> {
        // 使用 &*self 避免移动 self
        let mut config_value =
            serde_json::to_value(&*self).map_err(|e| format!("Serialization failed: {}", e))?;

        config_value[field] =
            serde_json::to_value(value).map_err(|e| format!("Value conversion failed: {}", e))?;

        // 使用临时变量避免所有权问题
        let new_config: Self = serde_json::from_value(config_value)
            .map_err(|e| format!("Deserialization failed: {}", e))?;

        *self = new_config;
        Ok(())
    }
}
