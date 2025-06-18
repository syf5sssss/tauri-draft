use chrono::{ DateTime, Local };
use tauri::{ command, AppHandle, State };
use tauri::Emitter;
use tauri::Manager;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{ AtomicBool, Ordering };

use crate::dto::ThreadState;

// 全局状态控制
static RUNNING: AtomicBool = AtomicBool::new(false);

#[command]
pub fn start_time(app_handle: AppHandle) {
    // 确保只有一个更新线程在运行
    if RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }
    let app_handle_clone = app_handle.clone();
    thread::spawn(move || {
        while RUNNING.load(Ordering::SeqCst) {
            let now: DateTime<Local> = Local::now();
            let formatted_now = now.format("%Y-%m-%d %H:%M:%S").to_string();
            if let Err(e) = app_handle_clone.emit("update_time", &formatted_now) {
                eprintln!("Failed to emit event: {}", e);
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        // 重置状态允许重新启动
        RUNNING.store(false, Ordering::SeqCst);
    });
}

#[command]
pub fn stop_time() {
    RUNNING.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn start_timer(app_handle: AppHandle, interval: u64) -> Result<(), String> {
    // 直接从 app_handle 获取状态
    let state = app_handle.state::<ThreadState>();

    let mut timer_running = state.timer_running.lock().unwrap();
    if *timer_running {
        return Err("Timer is already running".into());
    }

    *timer_running = true;
    *state.interval.lock().unwrap() = interval;

    // 正确克隆状态：使用 AppHandle 在后台线程中重新获取状态
    let app_handle_clone = app_handle.clone();

    thread::spawn(move || {
        // 在后台线程中重新获取状态
        let state = app_handle_clone.state::<ThreadState>();

        loop {
            let should_stop = {
                let running = state.timer_running.lock().unwrap();
                !*running
            };
            if should_stop {
                break;
            }

            let (value, timestamp) = {
                let value = state.value.lock().unwrap().clone();
                let now: DateTime<Local> = Local::now();
                let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
                let timeclone = timestamp.clone();
                let mut shared_value = state.timestamp.lock().unwrap();
                *shared_value = timeclone;
                (value, timestamp)
            };

            let event_data = serde_json::json!({ "value": value, "timestamp": timestamp });
            if app_handle_clone.emit("value_updated", &event_data).is_err() {
                break;
            }

            let sleep_duration = {
                let interval_lock = state.interval.lock().unwrap();
                Duration::from_millis(*interval_lock)
            };

            thread::sleep(sleep_duration);
        }
    });

    Ok(())
}

// 停止定时器
#[tauri::command]
pub fn stop_timer(thread_state: State<'_, ThreadState>) -> Result<(), String> {
    let mut timer_running = thread_state.timer_running.lock().unwrap();
    *timer_running = false;
    Ok(())
}

// 获取共享值
#[tauri::command]
pub fn get_shared_value(thread_state: State<'_, ThreadState>) -> Result<String, String> {
    let shared_value = thread_state.value.lock().unwrap();
    Ok(shared_value.clone())
}

// 设置共享值
#[tauri::command]
pub fn set_shared_value(thread_state: State<'_, ThreadState>, value: String) -> Result<(), String> {
    let mut shared_value = thread_state.value.lock().unwrap();
    *shared_value = value;
    Ok(())
}

// 获取最后更新时间
#[tauri::command]
pub fn get_last_update(thread_state: State<'_, ThreadState>) -> Result<String, String> {
    let last_updated = thread_state.timestamp.lock().unwrap();
    Ok(last_updated.clone())
}

#[tauri::command]
pub fn progress_update(app_handle: AppHandle) {
    let app_handle_clone = app_handle.clone();
    let mut vector = vec![(2, "操作准备中"), (11, "下班回家"), (24, "钉钉打卡"), (32, "自行车行驶")];
    vector.push((44, "菜市场买菜"));
    vector.push((54, "自行车停放"));
    vector.push((64, "收快递回家"));
    vector.push((77, "播放音乐"));
    vector.push((87, "洗菜做饭"));
    vector.push((92, "打开电视并吃饭"));
    vector.push((100, "准备睡觉"));

    for i in &vector {
        let now: DateTime<Local> = Local::now();
        let formatted_now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let pro = format!("{},{}", formatted_now, i.1);
        // println!("进度: {}%, 操作: {}", i.0, i.1);
        if let Err(e) = app_handle_clone.emit("progress_msg", &pro) {
            eprintln!("Failed to emit event: {}", e);
        }
        thread::sleep(Duration::from_millis(10));
        if let Err(e) = app_handle_clone.emit("progress_update", i.0) {
            eprintln!("Failed to emit event: {}", e);
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
