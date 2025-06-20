use tauri::{ AppHandle, Emitter, State };
use tokio::net::{ TcpListener, TcpStream };
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::sync::{ broadcast, mpsc, RwLock };
use serde_json::{ Value, json };
use std::sync::Arc;
use std::collections::HashMap;
use std::str::FromStr;

// 服务器状态
pub struct TcpServerState {
    pub running: bool,
    pub listener: Option<TcpListener>,
    pub tx: Option<Arc<broadcast::Sender<String>>>, // 广播通道
    pub clients: Arc<RwLock<HashMap<std::net::SocketAddr, broadcast::Sender<String>>>>, // 客户端映射
    pub shutdown_tx: Option<mpsc::Sender<()>>,
}

impl Default for TcpServerState {
    fn default() -> Self {
        Self {
            running: false,
            listener: None,
            tx: None,
            clients: Arc::new(RwLock::new(HashMap::new())),
            shutdown_tx: None,
        }
    }
}

#[tauri::command]
pub async fn start_tcp_server(state: State<'_, Arc<RwLock<TcpServerState>>>, app_handle: AppHandle, ip: String, port: u32) -> Result<(), String> {
    // 检查服务器是否已运行
    {
        let state = state.read().await;
        if state.running {
            if let Err(e) = app_handle.emit("server_msg", "Server is already running") {
                eprintln!("Failed to emit event: {}", e);
            }
            return Err("Server is already running".into());
        }
    }

    let constr = format!("{}:{}", ip, port);
    println!("Starting server on {}", constr);

    // 创建服务器状态
    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(tx);
    let listener = TcpListener::bind(&constr).await.map_err(|e| {
        if let Err(ea) = app_handle.emit("server_msg", format!("Failed to bind to {}: {}", constr, e)) {
            eprintln!("Failed to emit event: {}", ea);
        }
        format!("Failed to bind to {}: {}", constr, e)
    })?;
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    // 更新状态
    {
        let mut state = state.write().await;
        state.running = true;
        state.listener = Some(listener);
        state.tx = Some(Arc::clone(&tx));
        state.shutdown_tx = Some(shutdown_tx);
    }

    // 启动服务器主循环
    let tx_clone = Arc::clone(&tx);
    let clients_clone = {
        let state = state.read().await;
        Arc::clone(&state.clients)
    };

    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        // 获取监听器
        let listener = {
            let mut state = state_clone.write().await;
            state.listener
                .take()
                .ok_or_else(|| String::from("Server listener not initialized"))
                .unwrap()
        };

        if let Err(ea) = app_handle.emit("server_msg", format!("{}", "Server listener")) {
            eprintln!("Failed to emit event: {}", ea);
        }
        server_main_loop(app_handle.clone(), tx_clone, clients_clone, listener, shutdown_rx).await;

        // 服务器停止后更新状态
        let mut state = state_clone.write().await;
        state.running = false;
        state.tx = None;
        state.shutdown_tx = None;
        println!("Server stopped");
        if let Err(e) = app_handle.emit("server_msg", "Server stopped") {
            eprintln!("Failed to emit event: {}", e);
        }
    });

    Ok(())
}

async fn server_main_loop(app_handle: AppHandle, tx: Arc<broadcast::Sender<String>>, clients: Arc<RwLock<HashMap<std::net::SocketAddr, broadcast::Sender<String>>>>, listener: TcpListener, mut shutdown_rx: mpsc::Receiver<()>) {
    // 使用 accept() 替代 incoming()
    let shutdown_notified = false;
    // 创建客户端关闭通道
    let (client_shutdown_tx, mut client_shutdown_rx) = mpsc::channel::<()>(1);

    loop {
        // 等待关闭信号或新连接
        tokio::select! {
            biased; // 优先处理关闭信号
            
            _ = shutdown_rx.recv() => {
                // 关闭服务器
                if !shutdown_notified {
                    println!("Shutting down server");
                    // 通知所有客户端关闭
                    let clients = clients.read().await;
                    for client_tx in clients.values() {
                        let _ = client_tx.send(json!({
                            "system": "server_shutdown"
                        }).to_string() + "\r\n");
                    }
                    break;
                }
            }
            
            // 处理客户端关闭信号
            _ = client_shutdown_rx.recv() => {
                // 此处可添加自定义客户端关闭处理
                // println!("a client connect close");
            }

            result = listener.accept() => {
                match result {
                    Ok((stream, addr)) => {
                        println!("New client connected: {}", addr);
                        let app_handle_clone = app_handle.clone();
                        if let Err(e) = app_handle_clone.clone().emit("server_msg", format!("New client connected: {}", addr)) {
                            eprintln!("Failed to emit event: {}", e);
                        }
                        if let Err(e) = app_handle_clone.emit("conn_add", format!("{}", addr)) {
                            eprintln!("Failed to emit event: {}", e);
                        }
                        // 克隆共享资源
                        let tx_clone = Arc::clone(&tx);
                        let clients_clone = Arc::clone(&clients);
                        // 克隆客户端关闭通道
                        let client_shutdown_tx_clone = client_shutdown_tx.clone();
                        
                        tokio::spawn(async move {
                            handle_client(app_handle_clone, stream, addr, tx_clone, clients_clone, client_shutdown_tx_clone).await;
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        }
    }

    // 关闭所有客户端连接
    println!("Closing all clients...");
    let mut clients = clients.write().await;
    for (addr, client_tx) in clients.drain() {
        println!("Closing client: {}", addr);
        // 发送关闭通知
        let _ = client_tx.send(json!({
            "system": "server_shutdown",
            "message": "Server is shutting down"
        }).to_string() + "\r\n");
    }

    // 等待一段时间让客户端处理关闭通知
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    println!("All clients closed");
}

#[tauri::command]
pub async fn stop_tcp_server(state: State<'_, Arc<RwLock<TcpServerState>>>) -> Result<(), String> {
    // 检查服务器是否在运行
    {
        let state = state.read().await;
        if !state.running {
            return Err("Server is not running".into());
        }
    }

    // 发送关闭信号
    {
        let state = state.read().await;
        if let Some(tx) = &state.shutdown_tx {
            if tx.send(()).await.is_err() {
                eprintln!("Failed to send shutdown signal");
            }
        }
    }

    // 等待服务器完全停止
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 再次检查并清理可能残留的客户端
    let clients = {
        let state = state.read().await;
        Arc::clone(&state.clients)
    };

    let mut clients = clients.write().await;
    if !clients.is_empty() {
        println!("Cleaning up {} remaining clients", clients.len());
        clients.clear();
    }

    Ok(())
}

#[tauri::command]
pub async fn send_to_clients(state: State<'_, Arc<RwLock<TcpServerState>>>, message: String) -> Result<(), String> {
    // 验证JSON格式
    let _: Value = serde_json::from_str(&message).map_err(|e| format!("Invalid JSON: {}", e))?;

    // 获取广播发送器并立即尝试发送消息
    let result = {
        let state = state.read().await;

        // 检查服务器是否在运行并获取发送器
        let tx = state.tx.as_ref().ok_or_else(|| "Server is not running or not initialized".to_string())?;

        // 发送消息给所有客户端
        let message_with_delimiter = message.clone() + "\r\n";
        tx.send(message_with_delimiter)
    };

    // 处理发送结果
    match result {
        Ok(_) => Ok(()),
        Err(broadcast::error::SendError(_)) => {
            // 通道已关闭，检查服务器状态
            let state = state.read().await;
            if !state.running {
                Err("Server is not running".into())
            } else {
                Err("Server is in the process of shutting down".into())
            }
        }
    }
}

// 新增：向指定客户端发送消息
#[tauri::command]
pub async fn send_to_client(
    state: State<'_, Arc<RwLock<TcpServerState>>>,
    client_addr: String, // 客户端地址，如 "127.0.0.1:12345"
    message: String
) -> Result<(), String> {
    // 验证JSON格式
    let _: Value = serde_json::from_str(&message).map_err(|e| format!("Invalid JSON: {}", e))?;

    // 解析客户端地址
    let addr = std::net::SocketAddr::from_str(&client_addr).map_err(|e| format!("Invalid client address: {}", e))?;

    // 获取状态
    let state_guard = state.read().await;

    // 检查服务器是否在运行
    if !state_guard.running {
        return Err("Server is not running".into());
    }

    // 获取客户端映射
    let clients = state_guard.clients.read().await;

    // 查找指定客户端的发送器
    if let Some(client_tx) = clients.get(&addr) {
        // 发送消息到指定客户端
        let message_with_delimiter = message + "\r\n";
        if client_tx.send(message_with_delimiter).is_err() {
            return Err(format!("Failed to send to client {}", addr));
        }
        Ok(())
    } else {
        Err(format!("Client {} not found", addr))
    }
}

// 处理单个客户端连接
async fn handle_client(
    app_handle: AppHandle,
    stream: TcpStream,
    addr: std::net::SocketAddr,
    tx: Arc<broadcast::Sender<String>>, // 广播通道
    clients: Arc<RwLock<HashMap<std::net::SocketAddr, broadcast::Sender<String>>>>, // 客户端映射
    client_shutdown_tx: mpsc::Sender<()> // 客户端关闭通知通道
) {
    // println!("Handling client connection from {}", addr);

    // 为客户端创建一个独立的发送通道
    let (client_tx, mut client_rx) = broadcast::channel(100);

    // 关键修复：订阅主广播通道
    let mut main_rx = tx.subscribe();

    // 将客户端添加到客户端列表
    {
        let mut clients = clients.write().await;
        clients.insert(addr, client_tx.clone());
    }

    // 拆分流为读写部分
    let (mut reader, mut writer) = tokio::io::split(stream);

    // 创建一个缓冲区用于累积数据
    let mut buffer = Vec::new();
    // 启动一个任务处理来自服务器的消息
    let writer_app_handle = app_handle.clone();
    let writer_task = tokio::spawn(async move {
        // println!("Starting writer task for client {}", addr);
        loop {
            tokio::select! {
                // 处理主广播通道的消息
                msg = main_rx.recv() => {
                    match msg {
                        Ok(msg) => {
                            // 检查消息是否是系统关闭通知
                            if msg.contains("server_shutdown") {
                                println!("Received shutdown notification for client {}", addr);
                                break;
                            }

                            println!("Sending broadcast message to client {}: {}", addr, msg.trim_end_matches("\r\n"));

                            if let Err(e) = writer_app_handle.emit("server_data", format!("{}", msg)) {
                                eprintln!("Failed to emit event: {}", e);
                            }
                            // 发送消息到客户端
                            if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                eprintln!("Error sending to {}: {}", addr, e);
                                break;
                            }
                            if let Err(e) = writer.write_all(b"\r\n").await {
                                eprintln!("Error sending delimiter to {}: {}", addr, e);
                                break;
                            }
                        }
                        Err(_) => {
                            println!("Main broadcast channel closed for client {}", addr);
                            break;
                        }
                    }
                }
                // 处理客户端专有通道的消息（定向消息）
                msg = client_rx.recv() => {
                    match msg {
                        Ok(msg) => {
                            // 检查消息是否是系统关闭通知
                            if msg.contains("server_shutdown") {
                                println!("Received shutdown notification for client {}", addr);
                                break;
                            }

                            println!("Sending private message to client {}: {}", addr, msg.trim_end_matches("\r\n"));

                            if let Err(e) = writer_app_handle.emit("server_data", format!("{}", msg)) {
                                eprintln!("Failed to emit event: {}", e);
                            }
                            // 发送消息到客户端
                            if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                eprintln!("Error sending to {}: {}", addr, e);
                                break;
                            }
                            if let Err(e) = writer.write_all(b"\r\n").await {
                                eprintln!("Error sending delimiter to {}: {}", addr, e);
                                break;
                            }
                        }
                        Err(_) => {
                            println!("Private channel closed for client {}", addr);
                            break;
                        }
                    }
                }
            }
        }
        // println!("Writer task for client {} completed", addr);
    });

    // 主循环：读取客户端数据
    // println!("Starting reader loop for client {}", addr);
    loop {
        // 定义读取缓冲区
        let mut buf = vec![0; 1024];

        match reader.read(&mut buf).await {
            Ok(0) => {
                // 客户端关闭连接
                println!("Client {} disconnected", addr);
                if let Err(e) = app_handle.emit("server_msg", format!("Client {} disconnected", addr)) {
                    eprintln!("Failed to emit event: {}", e);
                }
                if let Err(e) = app_handle.emit("conn_del", format!("{}", addr)) {
                    eprintln!("Failed to emit event: {}", e);
                }
                break;
            }
            Ok(n) => {
                buffer.extend_from_slice(&buf[0..n]);

                // 处理消息
                while let Some(delimiter_pos) = find_delimiter(&buffer) {
                    // 提取完整消息
                    let message = String::from_utf8_lossy(&buffer[0..delimiter_pos]).into_owned();

                    // 从缓冲区移除已处理的消息
                    if delimiter_pos + 2 <= buffer.len() {
                        buffer = buffer[delimiter_pos + 2..].to_vec(); // +2 跳过 \r\n
                    } else {
                        // 防止溢出，清空缓冲区
                        eprintln!("Invalid delimiter position, clearing buffer for client {}", addr);
                        buffer.clear();
                    }

                    // 解析JSON
                    match serde_json::from_str::<Value>(&message) {
                        Ok(json_data) => {
                            println!("Received message from {}: {:?}", addr, json_data);
                            if let Err(e) = app_handle.emit("server_data", format!("{}", json_data)) {
                                eprintln!("Failed to emit event: {}", e);
                            }
                            // 不再自动回复客户端消息
                        }
                        Err(e) => {
                            eprintln!("Failed to parse JSON from {}: {}", addr, e);

                            // 发送错误响应
                            let error_response = json!({
                                "error": "Invalid JSON",
                                "details": e.to_string()
                            }).to_string();

                            if let Err(e) = client_tx.send(error_response + "\r\n") {
                                eprintln!("Failed to send error response: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", addr, e);
                break;
            }
        }
    }

    // 清理：从客户端列表中移除
    {
        let mut clients = clients.write().await;
        clients.remove(&addr);
    }

    // 发送客户端关闭通知
    let _ = client_shutdown_tx.send(()).await;

    println!("Client {} fully disconnected", addr);
    drop(writer_task);
}

// 查找消息分隔符 \r\n 的位置
pub fn find_delimiter(buffer: &[u8]) -> Option<usize> {
    // 避免空缓冲区导致的溢出
    if buffer.len() < 2 {
        return None;
    }

    for i in 0..buffer.len() - 1 {
        if buffer[i] == b'\r' && buffer[i + 1] == b'\n' {
            return Some(i);
        }
    }
    None
}
