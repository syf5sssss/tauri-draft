use std::time::Duration;

use commands::connect_db;

pub mod dto;
pub mod dao;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async {
        // 等待数据库连接
        tokio::time::sleep(Duration::from_secs(2)).await;
        let res = connect_db("127.0.0.1", "sa", "Nanhui-380").await;
        println!("启动应用执行 {:?}", res);
    });
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(crate::commands::register_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
