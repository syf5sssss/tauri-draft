use std::{
    net::UdpSocket,
    sync::{ Arc, Mutex },
    thread,
    time::Duration, // 使用标准库的 Duration
};
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

    // 设置合理的读取超时时间（500毫秒）
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(500))) {
        eprintln!("Set read timeout failed: {}, continue anyway", e);
    }

    // 存储套接字
    *state.socket.lock().unwrap() = Some(socket.clone());

    println!("Server broadcast opened");
    // 克隆共享状态
    let shared_inner = Arc::clone(&state.inner);
    thread::spawn(move || {
        receive_broadcast_messages(socket, shared_inner);
    });

    Ok(())
}

// 接收广播消息的线程函数（优化粘包处理）
pub fn receive_broadcast_messages(socket: Arc<UdpSocket>, state: Arc<Mutex<InnerState>>) {
    let mut buffer = Vec::with_capacity(2048); // 更大的缓冲区
    let mut temp_buf = [0u8; 1024];
    let delimiter = b"\r\n";

    loop {
        // 检查运行状态
        let running = {
            let guard = state.lock().unwrap();
            guard.running
        };
        if !running {
            break;
        }

        match socket.recv_from(&mut temp_buf) {
            Ok((size, src)) => {
                buffer.extend_from_slice(&temp_buf[..size]);

                // 处理所有完整消息
                while let Some(pos) = buffer.windows(delimiter.len()).position(|w| w == delimiter) {
                    // 提取一条完整消息（不包括分隔符）
                    let message_bytes = buffer.drain(..pos).collect::<Vec<_>>();
                    buffer.drain(..delimiter.len()); // 移除分隔符

                    if let Ok(message) = String::from_utf8(message_bytes.clone()) {
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
                    } else {
                        println!("Received invalid UTF-8 data from {}", src);
                    }
                }

                // 防止缓冲区无限增长（设置最大长度）
                if buffer.len() > 8192 {
                    println!("Buffer overflow, clearing buffer");
                    buffer.clear();
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                // 超时或阻塞是预期行为，继续循环
                continue;
            }
            Err(e) => {
                // 只打印非预期错误
                if e.kind() != std::io::ErrorKind::ConnectionReset {
                    eprintln!("Receive error: {:?}", e);
                }
                // 发生错误时清空缓冲区
                buffer.clear();
            }
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

// 发送广播消息（添加粘包处理）
#[tauri::command]
pub async fn send_broadcast_message(state: State<'_, BroadcastState>, message: String, port: u16) -> Result<(), String> {
    // 检查运行状态
    {
        let guard = state.inner.lock().unwrap();
        if !guard.running {
            return Err("Broadcast service is not running".into());
        }
    }

    // 添加分隔符 \r\n
    let full_message = format!("{}\r\n", message);

    let socket_guard = state.socket.lock().unwrap();
    let socket = match &*socket_guard {
        Some(s) => s,
        None => {
            return Err("Socket not available".into());
        }
    };

    let broadcast_addr = format!("255.255.255.255:{}", port);
    socket.send_to(full_message.as_bytes(), broadcast_addr).map_err(|e| format!("Send failed: {}", e))?;

    println!("Sent broadcast: {}", message);
    Ok(())
}
