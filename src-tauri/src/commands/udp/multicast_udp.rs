use std::{ net::{ SocketAddr, Ipv4Addr }, sync::Arc, collections::VecDeque };
use tauri::{ AppHandle, Emitter, State };
use tokio::{ net::UdpSocket, sync::{ mpsc, Mutex } };
use serde::{ Deserialize, Serialize };

// 消息结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UdpMessage {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

// UDP服务状态
pub struct UdpService {
    socket: Option<Arc<UdpSocket>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

// 状态管理 - 使用 tokio::sync::Mutex
pub struct MulticastState {
    udp_service: Mutex<UdpService>,
}

impl Default for MulticastState {
    fn default() -> Self {
        MulticastState {
            udp_service: Mutex::new(UdpService {
                socket: None,
                shutdown_tx: None,
            }),
        }
    }
}

#[tauri::command]
pub async fn open_udp_service(
    app: AppHandle,
    state: State<'_, MulticastState>,
    addr: String, // 绑定地址，如 "0.0.0.0:8888"
    multicast_addr: String // 组播地址，如 "232.252.252.252"
) -> Result<(), String> {
    let mut service = state.udp_service.lock().await;

    if service.socket.is_some() {
        return Err("Service already running".into());
    }

    // 创建套接字
    let socket = UdpSocket::bind(&addr).await.map_err(|e| format!("Bind failed: {}", e))?;

    // 加入组播
    let multicast_ip: Ipv4Addr = multicast_addr.parse().map_err(|_| format!("Invalid multicast address: {}", multicast_addr))?;

    // 使用Tokio原生方法加入组播
    socket.join_multicast_v4(multicast_ip, Ipv4Addr::UNSPECIFIED).map_err(|e| format!("Join multicast failed: {}", e))?;

    // 设置TTL以支持跨路由器
    socket.set_ttl(32).map_err(|e| format!("Set TTL failed: {}", e))?;

    let socket = Arc::new(socket);

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
    service.shutdown_tx = Some(shutdown_tx);
    service.socket = Some(socket.clone());

    // 释放锁后再启动任务
    let app_handle = app.clone();

    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        let mut packet_queue = VecDeque::new();

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    println!("UDP service shutting down");
                    break;
                }
                result = socket.recv_from(&mut buffer) => {
                    match result {
                        Ok((size, src)) => {
                            let data = &buffer[..size];
                            process_data(
                                data,
                                &mut packet_queue,
                                &app_handle,
                                src
                            );
                        }
                        Err(e) => {
                            println!("Receive error: {}", e);
                        }
                    }
                }
            }
        }
    });

    println!("UDP service started on {} with multicast group {}", addr, multicast_addr);
    Ok(())
}

// 处理接收数据（修复版）
pub fn process_data(data: &[u8], packet_queue: &mut VecDeque<u8>, app: &AppHandle, src: SocketAddr) {
    packet_queue.extend(data);

    // 持续查找完整的 \r\n 分隔符
    while packet_queue.len() >= 2 {
        // 将队列数据转为连续切片
        packet_queue.make_contiguous();
        let slice = packet_queue.as_slices().0;

        // 查找 \r\n 位置
        if let Some(pos) = slice.windows(2).position(|w| w == b"\r\n") {
            // 提取完整消息（不包括 \r\n）
            let message_bytes = &slice[..pos];
            let message = String::from_utf8_lossy(message_bytes).into_owned();

            // 移除已处理数据（消息内容 + \r\n）
            packet_queue.drain(..pos + 2);

            let event_data = serde_json::json!({
                "message": message,
                "source": src.to_string()
            });

            println!("Received valid message: {}", event_data.to_string());
            app.emit("multicast-message", event_data).unwrap_or_else(|e| {
                println!("Emit failed: {}", e);
            });
        } else {
            // 未找到完整分隔符，等待更多数据
            break;
        }
    }
}

// 关闭UDP服务
#[tauri::command]
pub async fn close_udp_service(state: State<'_, MulticastState>) -> Result<(), String> {
    // 获取 shutdown_tx 后立即释放锁
    let shutdown_tx = {
        let mut service = state.udp_service.lock().await;
        service.shutdown_tx.take()
    };

    if let Some(tx) = shutdown_tx {
        tx.send(()).await.map_err(|e| format!("Send shutdown failed: {}", e))?;
    }

    // 再次获取锁来清理状态
    let mut service = state.udp_service.lock().await;
    service.socket = None;

    println!("UDP service stopped");
    Ok(())
}

// 发送UDP消息
#[tauri::command]
pub async fn send_udp_message(
    state: State<'_, MulticastState>,
    message: String, // 直接接收结构体
    target_addr: String
) -> Result<(), String> {
    // 只短暂持有锁来获取套接字
    let socket = {
        let service = state.udp_service.lock().await;
        service.socket.clone()
    };

    let socket = match socket {
        Some(s) => s,
        None => {
            return Err("UDP service not running".into());
        }
    };

    // 序列化消息并添加分隔符
    // let json_msg = serde_json::to_string(&message).map_err(|e| format!("JSON serialization failed: {}", e))?;
    let full_msg = format!("{}\r\n", &message);

    socket.send_to(full_msg.as_bytes(), &target_addr).await.map_err(|e| format!("Send failed: {}", e))?;

    println!("Sent message to {}", target_addr);
    Ok(())
}

// 发送组播消息（专用命令）
#[tauri::command]
pub async fn send_multicast_message(state: State<'_, MulticastState>, message: String, port: u16, multicast_addr: String) -> Result<(), String> {
    let target_addr = format!("{}:{}", multicast_addr, port);
    send_udp_message(state, message, target_addr).await
}
