use crate::commands::find_delimiter;
use serde_json::Value;
use std::sync::Arc;
use tauri::{command, AppHandle, Emitter, State};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

// 客户端状态
#[derive(Clone)]
pub struct TcpClientState {
    tx: Arc<Mutex<Option<mpsc::Sender<String>>>>,
}

impl Default for TcpClientState {
    fn default() -> Self {
        Self {
            tx: Arc::new(Mutex::new(None)),
        }
    }
}

#[command]
pub async fn tcp_client_connect(
    app_handle: AppHandle,
    client: State<'_, TcpClientState>,
    address: String,
) -> Result<(), String> {
    // 检查是否已连接
    {
        let tx_guard = client.tx.lock().await;
        if tx_guard.is_some() {
            let _ = app_handle.emit("client_msg", format!("已成功连接"));
            return Err("Already connected".into());
        }
    }

    // 连接到服务器
    let stream = TcpStream::connect(&address).await.map_err(|e| {
        let _ = app_handle.emit("client_msg", format!("连接失败: {}", e));
        format!("Connection failed: {}", e)
    })?;

    stream
        .set_nodelay(true)
        .map_err(|e| format!("Failed to set nodelay: {}", e))?;

    let _ = app_handle.emit("client_msg", "连接成功");

    // 分离读写
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    // 创建通道用于发送消息
    let (tx, mut rx) = mpsc::channel(100);

    // 更新客户端状态
    {
        let mut tx_guard = client.tx.lock().await;
        *tx_guard = Some(tx.clone());
    }

    // 启动发送任务
    tokio::spawn(async move {
        let mut writer = write_half;
        while let Some(message) = rx.recv().await {
            // 发送消息
            if let Err(e) = writer.write_all(message.as_bytes()).await {
                eprintln!("Error sending message: {}", e);
                break;
            }

            // 发送分隔符
            if let Err(e) = writer.write_all(b"\r\n").await {
                eprintln!("Error sending delimiter: {}", e);
                break;
            }
        }
    });

    // 启动接收任务
    let client_state = client.inner().clone();
    tokio::spawn(async move {
        let mut buffer = Vec::new();

        loop {
            let mut buf = vec![0; 1024];
            match reader.read(&mut buf).await {
                Ok(0) => {
                    // 连接关闭
                    eprintln!("Connection closed by server");
                    let _ = app_handle.emit("client_msg", "连接已关闭");
                    break;
                }
                Ok(n) => {
                    buffer.extend_from_slice(&buf[0..n]);

                    // 处理所有完整的消息
                    while let Some(delimiter_pos) = find_delimiter(&buffer) {
                        // 提取完整消息
                        let message =
                            String::from_utf8_lossy(&buffer[0..delimiter_pos]).into_owned();

                        // 从缓冲区移除已处理的消息
                        buffer = buffer[delimiter_pos + 2..].to_vec(); // +2 跳过 \r\n

                        // 解析JSON
                        match serde_json::from_str::<Value>(&message) {
                            Ok(json_data) => {
                                // 发送消息到前端
                                println!("received message: {}", json_data);
                                let _ = app_handle.emit("client_data", json_data);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse JSON: {}", e);
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
            let mut tx_guard = client_state.tx.lock().await;
            *tx_guard = None;
        }
    });

    Ok(())
}

#[command]
pub async fn disconnect(client: State<'_, TcpClientState>) -> Result<(), String> {
    // 清理连接
    {
        let mut tx_guard = client.tx.lock().await;
        *tx_guard = None;
    }
    Ok(())
}

#[command]
pub async fn send_message(
    client: State<'_, TcpClientState>,
    message: String,
) -> Result<(), String> {
    // 验证JSON格式
    let _: Value = serde_json::from_str(&message).map_err(|e| format!("Invalid JSON: {}", e))?;

    // 发送消息
    let tx_guard = client.tx.lock().await;
    let tx = tx_guard
        .as_ref()
        .ok_or_else(|| "Not connected".to_string())?;

    tx.send(message)
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    Ok(())
}
