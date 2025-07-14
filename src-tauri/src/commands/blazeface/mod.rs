use anyhow::Result;
use face_manager::{ detect_face_position, init, FaceRect };
use tauri::command;
use std::fs::write;
use std::path::PathBuf;

use base64::engine::general_purpose;
use base64::Engine as _;
use image::load_from_memory;

mod face_manager;

// 初始化
#[command]
pub async fn face_init() -> Result<(), String> {
    init().map_err(|e| e.to_string())
}

#[command]
pub async fn write_file(path: String, data: Vec<u8>) -> Result<(), String> {
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write(&path, data).map_err(|e| e.to_string())
}

// 调用face_manager中的人脸位置检测功能
#[command]
pub async fn cmd_detect_face_position(image_data: String) -> Result<Option<FaceRect>, String> {
    // 解码base64图像数据
    let image_bytes = general_purpose::STANDARD.decode(&image_data).map_err(|e| format!("解码失败: {}", e))?;

    // 加载图像并转换为Rgb8格式（与face_manager兼容）
    let img = load_from_memory(&image_bytes)
        .map_err(|e| format!("图像加载失败: {}", e))?
        .into_rgb8(); // 转换为RgbImage

    // 调用face_manager中的检测方法
    detect_face_position(img).map_err(|e| e.to_string())
}
