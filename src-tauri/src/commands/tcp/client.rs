use tauri::{ command, State, AppHandle };
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::sync::{ Mutex, mpsc };
use serde_json::{ Value, json };
use std::sync::Arc;
use std::time::Duration;

// 客户端状态
struct TcpClient {
    stream: Mutex<Option<(TcpStream, BufReader<TcpStream>)>>,
    tx: Mutex<Option<mpsc::Sender<String>>>,
}

impl Default for TcpClient {
    fn default() -> Self {
        Self {
            stream: Mutex::new(None),
            tx: Mutex::new(None),
        }
    }
}

#[command]
pub async fn tcp_client_connect(app_handle: AppHandle, client: State<'_, TcpClient>, address: String) -> Result<(), String> {
    // 检查是否已连接
    let stream_guard = client.stream.lock().await;
    if stream_guard.is_some() {
        return Err("Already connected".into());
    }
    drop(stream_guard); // 释放锁

    // 连接到服务器
    let stream = TcpStream::connect(&address).await.map_err(|e| format!("Connection failed: {}", e))?;

    // 设置非阻塞模式
    stream.set_nodelay(true).map_err(|e| format!("Failed to set nodelay: {}", e))?;

    // 创建读写分离的流
    let reader = BufReader::new(stream.try_clone().map_err(|e| format!("Failed to clone stream: {}", e))?);

    // 创建通道用于发送消息
    let (tx, mut rx) = mpsc::channel(100);

    // 更新客户端状态
    {
        let mut stream_guard = client.stream.lock().await;
        *stream_guard = Some((stream, reader));

        let mut tx_guard = client.tx.lock().await;
        *tx_guard = Some(tx.clone());
    }

    // 启动发送任务
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let mut stream_guard = client.stream.lock().await;
            if let Some((stream, _)) = &mut *stream_guard {
                // 发送消息
                if let Err(e) = stream.write_all(message.as_bytes()).await {
                    eprintln!("Error sending message: {}", e);
                    // 发送断开连接事件
                    let _ = app_handle_clone.emit("disconnected", ());
                    break;
                }

                // 发送分隔符
                if let Err(e) = stream.write_all(b"\r\n").await {
                    eprintln!("Error sending delimiter: {}", e);
                    // 发送断开连接事件
                    let _ = app_handle_clone.emit("disconnected", ());
                    break;
                }
            } else {
                break;
            }
        }
    });

    // 启动接收任务
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        let mut buffer = Vec::new();

        loop {
            let mut stream_guard = client.stream.lock().await;
            let (_, reader) = match &mut *stream_guard {
                Some((_, reader)) => reader,
                None => {
                    break;
                }
            };

            // 读取数据
            let mut buf = vec![0; 1024];
            match reader.read(&mut buf).await {
                Ok(n) if n == 0 => {
                    // 连接关闭
                    eprintln!("Connection closed by server");
                    break;
                }
                Ok(n) => {
                    // 将读取的数据追加到缓冲区
                    buffer.extend_from_slice(&buf[0..n]);

                    // 处理所有完整的消息
                    while let Some(delimiter_pos) = find_delimiter(&buffer) {
                        // 提取完整消息
                        let message = String::from_utf8_lossy(&buffer[0..delimiter_pos]).into_owned();

                        // 从缓冲区移除已处理的消息
                        buffer = buffer[delimiter_pos + 2..].to_vec(); // +2 跳过 \r\n

                        // 解析JSON
                        match serde_json::from_str::<Value>(&message) {
                            Ok(json_data) => {
                                // 发送消息到前端
                                let _ = app_handle_clone.emit("message_received", json_data);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse JSON: {}", e);
                                let error_response = json!({
                                    "error": "Invalid JSON received",
                                    "details": e.to_string()
                                });
                                let _ = app_handle_clone.emit("message_received", error_response);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }

        // 清理连接
        {
            let mut stream_guard = client.stream.lock().await;
            *stream_guard = None;

            let mut tx_guard = client.tx.lock().await;
            *tx_guard = None;
        }

        // 发送断开连接事件
        let _ = app_handle_clone.emit("disconnected", ());
    });

    Ok(())
}

#[command]
pub async fn disconnect(client: State<'_, TcpClient>) -> Result<(), String> {
    // 清理连接
    {
        let mut stream_guard = client.stream.lock().await;
        *stream_guard = None;

        let mut tx_guard = client.tx.lock().await;
        *tx_guard = None;
    }

    Ok(())
}

#[command]
pub async fn send_message(client: State<'_, TcpClient>, message: String) -> Result<(), String> {
    // 验证JSON格式
    let _: Value = serde_json::from_str(&message).map_err(|e| format!("Invalid JSON: {}", e))?;

    // 发送消息
    let tx_guard = client.tx.lock().await;
    let tx = tx_guard.as_ref().ok_or_else(|| "Not connected".to_string())?;

    tx.send(message).await.map_err(|e| format!("Failed to send message: {}", e))?;

    Ok(())
}

// 查找消息分隔符 \r\n 的位置
pub fn find_delimiter(buffer: &[u8]) -> Option<usize> {
    for i in 0..buffer.len() - 1 {
        if buffer[i] == b'\r' && buffer[i + 1] == b'\n' {
            return Some(i);
        }
    }
    None
}

// fn main() {
//     tauri::Builder::default()
//         .manage(TcpClient::default())
//         .invoke_handler(tauri::generate_handler![connect, disconnect, send_message])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
