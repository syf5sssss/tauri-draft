use std::env;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_exe_dir() -> Result<String, String> {
    // 构建目标目录路径
    let current_dir = env::current_dir();
    if let Ok(dir) = &current_dir {
        println!("当前运行目录的地址: {:?}", dir.to_str());
        Ok(dir.to_string_lossy().into_owned())
    } else {
        return Err(format!("无法获取应用运行目录"));
    }
}

#[command]
pub fn get_pictures_dir(app: AppHandle) -> Result<String, String> {
    let pictures_dir = app.path().picture_dir();
    if let Ok(dir) = &pictures_dir {
        println!("图片目录的地址: {:?}", dir.to_str());
        Ok(dir.to_string_lossy().into_owned())
    } else {
        return Err(format!("无法获取图片目录"));
    }
}

#[command]
pub fn get_documents_dir(app: AppHandle) -> Result<String, String> {
    let documents_dir = app.path().document_dir();

    if let Ok(dir) = &documents_dir {
        println!("文档目录的地址: {:?}", dir.to_str());
        Ok(dir.to_string_lossy().into_owned())
    } else {
        return Err(format!("无法获取文档目录"));
    }
}
