use commands::{ connect_db, AppState, TcpServerState, TcpClientState };
use dto::ThreadState;
use tauri_plugin_autostart::MacosLauncher;

pub mod commands;
pub mod dao;
pub mod dto;
pub mod util;
use log::info;
use tokio::sync::RwLock;
use util::AppConfig;
use std::{ fs::{ self, OpenOptions }, io, path::PathBuf };
use std::path::Path;
use std::time::Duration;
use chrono::{ Local, Timelike };
use std::sync::{ Arc, Mutex };
use tauri::Manager;

fn setup_logger(target_dir: &PathBuf) -> Result<(), fern::InitError> {
    let now = Local::now();
    let log_path = target_dir
        .join("Draft")
        .join(format!("{}", now.format("%Y")))
        .join(format!("{}-{}", now.format("%m"), now.format("%d")))
        .join(format!("draft-{}.log", now.format("%Y%m%d-%H")));
    if let Err(e) = rotate_logs(&log_path) {
        println!("日志轮转失败: {}", e);
    }
    // 打开日志文件
    let log_file = OpenOptions::new().append(true).create(true).open(&log_path)?;

    // 配置日志系统
    fern::Dispatch
        ::new()
        .format(|out, message, record| { out.finish(format_args!("[{}][{}][{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), record.target(), record.level(), message)) })
        .level(log::LevelFilter::Info)
        .level_for("tao::platform_impl::platform::event_loop::runner", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::Output::writer(Box::new(log_file), "\n"))
        .apply()?;

    let thread_log_path = log_path.clone();
    // 启动后台线程检查整点
    std::thread::spawn(move || {
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
            if let Err(e) = rotate_logs(&thread_log_path) {
                println!("日志轮转失败: {}", e);
            }
        }
    });

    Ok(())
}

fn rotate_logs(log_path: &PathBuf) -> io::Result<()> {
    let path = Path::new(log_path);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .manage(TcpClientState::default())
        .manage(ThreadState::default())
        .manage(Arc::new(RwLock::new(TcpServerState::default())))
        .setup(|app| {
            let document_dir = app.path().document_dir();
            if let Ok(dir) = &document_dir {
                setup_logger(dir).expect("Failed to initialize logger");
            }
            info!("应用程序启动-日志开启");

            let cp = AppConfig::config_path(app.handle());
            info!("配置文件地址: {:?}", cp);
            let app_config = AppConfig::load(app.handle()).unwrap_or_else(|e| {
                eprintln!("Error loading config: {}, using default", e);
                AppConfig::default()
            });
            app.manage(AppState {
                config: Arc::new(Mutex::new(app_config)),
            });
            // 获取配置的克隆（而不是持有锁）
            let config_clone;
            {
                let app_state = app.state::<AppState>();
                let config = app_state.config.lock().unwrap();
                info!("配置加载成功: {:?}", config);
                config_clone = config.clone(); // 克隆配置数据
            } // 锁在这里释放

            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(Duration::from_secs(2)).await;
                let res = connect_db(&config_clone.ip, &config_clone.username, &config_clone.password).await;
                info!("数据库启动 {:?}", res);
            });

            Ok(())
        })
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
