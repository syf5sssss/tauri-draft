use chrono::{DateTime, Local};
use serde::Serialize;
use std::sync::Mutex;

// 1. 核心状态（原有的共享数据）
#[derive(Default)]
pub struct CoreState {
    pub shared_value: Mutex<String>,
    pub last_updated: Mutex<Option<DateTime<Local>>>,
}

// 2. 定时器状态
#[derive(Default)]
pub struct TimerState {
    pub timer_running: Mutex<bool>,
    pub interval: Mutex<u64>, // 新增：定时器间隔（毫秒）
}

// 3. 配置状态（例如从文件加载的参数）
#[derive(Default)]
pub struct ConfigState {
    pub api_endpoint: Mutex<String>,
    pub max_retries: Mutex<u32>,
    pub log_level: Mutex<String>,
    // ... 其他配置参数
}

// 4. 用户会话状态
#[derive(Default)]
pub struct SessionState {
    pub user_id: Mutex<Option<String>>,
    pub token: Mutex<Option<String>>,
    // ... 其他会话数据
}

// 事件结构
#[derive(Clone, Serialize)]
pub struct UpdateEvent {
    pub value: String,
    pub timestamp: String,
}

#[derive(Default)]
pub struct ThreadState {
    pub value: Mutex<String>,
    pub timestamp: Mutex<String>,
    pub timer_running: Mutex<bool>,
    pub interval: Mutex<u64>,
}
