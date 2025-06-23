use std::{ net::UdpSocket, sync::{ Arc, Mutex }, thread };
use tauri::{ AppHandle, Emitter, State };
use serde::Serialize;

// 内部状态结构
pub struct InnerState {
    running: bool,
    app_handle: Option<AppHandle>,
}

// 应用状态管理
pub struct BroadcastState {
    socket: Mutex<Option<Arc<UdpSocket>>>,
    inner: Arc<Mutex<InnerState>>, // 共享状态使用 Arc 封装
}

impl Default for BroadcastState {
    fn default() -> Self {
        BroadcastState {
            socket: Mutex::new(None),
            inner: Arc::new(
                Mutex::new(InnerState {
                    running: false,
                    app_handle: None,
                })
            ),
        }
    }
}

// 发送到前端的事件结构
#[derive(Clone, Serialize)]
pub struct BroadcastMessage {
    message: String,
    source: String,
}

// 打开广播服务
#[tauri::command]
pub async fn open_broadcast_service(app: AppHandle, state: State<'_, BroadcastState>, port: u16) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    if inner.running {
        return Err("Broadcast service is already running".into());
    }

    // 存储应用句柄
    inner.app_handle = Some(app.clone());
    inner.running = true;
    drop(inner); // 提前释放锁

    // 创建UDP套接字
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            return Err(format!("Bind failed: {}", e));
        }
    };

    if let Err(e) = socket.set_broadcast(true) {
        return Err(format!("Enable broadcast failed: {}", e));
    }

    // 存储套接字
    *state.socket.lock().unwrap() = Some(socket.clone());

    // 克隆共享状态
    let shared_inner = Arc::clone(&state.inner);
    thread::spawn(move || {
        receive_broadcast_messages(socket, shared_inner);
    });

    Ok(())
}

// 接收广播消息的线程函数
pub fn receive_broadcast_messages(socket: Arc<UdpSocket>, state: Arc<Mutex<InnerState>>) {
    let mut buf = [0; 1024];

    loop {
        // 检查运行状态（避免长时间持有锁）
        let running = {
            let guard = state.lock().unwrap();
            guard.running
        };
        if !running {
            break;
        }

        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let message = String::from_utf8_lossy(&buf[..size]).into_owned();
                println!("Received broadcast: {} from {}", message, src);

                // 发送事件到前端
                let guard = state.lock().unwrap();
                if let Some(app) = &guard.app_handle {
                    let event_data = BroadcastMessage {
                        message: message.clone(),
                        source: src.to_string(),
                    };
                    let _ = app.emit("broadcast-message", event_data);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => eprintln!("Receive error: {}", e),
        }
    }
    println!("Broadcast receiver thread stopped");
}

// 关闭广播服务
#[tauri::command]
pub async fn close_broadcast_service(state: State<'_, BroadcastState>) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    if !inner.running {
        return Err("Broadcast service is not running".into());
    }

    inner.running = false;
    inner.app_handle = None;
    *state.socket.lock().unwrap() = None;

    println!("Broadcast service stopped");
    Ok(())
}

// 发送广播消息
#[tauri::command]
pub async fn send_broadcast_message(state: State<'_, BroadcastState>, message: String, port: u16) -> Result<(), String> {
    // 检查运行状态
    {
        let guard = state.inner.lock().unwrap();
        if !guard.running {
            return Err("Broadcast service is not running".into());
        }
    }

    let socket_guard = state.socket.lock().unwrap();
    let socket = match &*socket_guard {
        Some(s) => s,
        None => {
            return Err("Socket not available".into());
        }
    };

    let broadcast_addr = format!("255.255.255.255:{}", port);
    socket.send_to(message.as_bytes(), broadcast_addr).map_err(|e| format!("Send failed: {}", e))?;

    println!("Sent broadcast: {}", message);
    Ok(())
}
