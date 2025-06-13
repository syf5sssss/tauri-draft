use commands::connect_db;
use tauri_plugin_autostart::MacosLauncher;

pub mod commands;
pub mod dao;
pub mod dto;
pub mod util;
use log::{ info, error };
use std::{ fs::{ self, OpenOptions }, io };
use std::path::Path;
use std::time::Duration;
use chrono::{ Local, Timelike };

fn setup_logger() -> Result<(), fern::InitError> {
    let now = Local::now();
    let log_path = format!("{}/{}-{}/draft-{}.log", now.format("%Y"), now.format("%m"), now.format("%d"), now.format("%Y%m%d-%H"));
    if let Err(e) = rotate_logs() {
        println!("日志轮转失败: {}", e);
    }

    // 打开日志文件
    let log_file = OpenOptions::new().append(true).create(true).open(&log_path)?;

    // 配置日志系统
    fern::Dispatch
        ::new()
        .format(|out, message, record| { out.finish(format_args!("[{}][{}][{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), record.target(), record.level(), message)) })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::Output::writer(Box::new(log_file), "\n"))
        .apply()?;

    // 启动后台线程检查整点
    std::thread::spawn(|| {
        loop {
            let now = Local::now();
            let sleep_seconds = if now.minute() == 0 && now.second() == 0 {
                // 整点，等待下一小时
                3600
            } else {
                // 计算到下一个整点的秒数
                (60 - now.minute()) * 60 - now.second()
            };

            std::thread::sleep(Duration::from_secs(sleep_seconds as u64));

            // 触发轮转
            if let Err(e) = rotate_logs() {
                println!("日志轮转失败: {}", e);
            }
        }
    });

    Ok(())
}

fn rotate_logs() -> io::Result<()> {
    let d = Local::now();
    let log_path = format!("{}/{}-{}/draft-{}.log", d.format("%Y"), d.format("%m"), d.format("%d"), d.format("%Y%m%d-%H"));

    let path = Path::new(&log_path);
    println!("轮转了一次：{}", log_path);
    // 一次性创建目录和文件
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // 如果文件不存在则创建
    if !path.exists() {
        OpenOptions::new().create(true).write(true).open(path)?;
    }

    Ok(())
}

// fn setup_logger() -> Result<(), fern::InitError> {
//     // 创建日志轮转器 (保留5个10MB的日志文件)
//     let log_rotator = FileRotate::new(
//         "draft",
//         AppendCount::new(500), // 保留5个历史文件
//         // ContentLimit::Bytes(10 * 1024 * 1024), // 10MB轮转
//         ContentLimit::Bytes(1024), // 10MB轮转
//         #[cfg(not(windows))] file_rotate::compression::Compression
//             // 非Windows系统添加压缩
//             ::OnRotate(1),
//         #[cfg(windows)] // Windows不需要压缩
//         file_rotate::compression::Compression::None,
//         None // 不添加时间戳后缀
//     );

//     // 配置日志系统
//     fern::Dispatch
//         ::new()
//         .format(|out, message, record| { out.finish(format_args!("[{}][{}][{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), record.target(), record.level(), message)) })
//         .level(log::LevelFilter::Debug) // 全局日志级别
//         .chain(std::io::stdout()) // 输出到控制台
//         .chain(fern::Output::writer(Box::new(log_rotator), "\n")) // 输出到轮转文件
//         .apply()?;

//     Ok(())
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logger().expect("Failed to initialize logger");

    info!("应用程序启动");
    error!("示例错误日志");
    tauri::async_runtime::spawn(async {
        // 等待数据库连接
        tokio::time::sleep(Duration::from_secs(2)).await;
        let res = connect_db("127.0.0.1", "sa", "Nanhui-380").await;
        println!("启动应用执行 {:?}", res);
    });

    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(crate::commands::register_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
