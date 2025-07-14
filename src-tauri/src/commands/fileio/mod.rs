use anyhow::Result;
use chrono::{DateTime, Local};
use serde::Serialize;
use std::collections::HashSet;
use std::process::Command;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};
use walkdir::WalkDir;
// 文件/目录类型
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
}

// 前端需要的节点数据格式（增强版）
#[derive(Serialize, Debug)]
pub struct NodeData {
    pub name: String,
    pub path: String,           // 添加完整路径
    pub size: u64,              // 原始字节大小
    pub formatted_size: String, // 格式化后的大小
    pub entry_type: EntryType,  // 使用枚举类型
    pub file_type: String,      // 详细文件类型
    pub created: String,        // 创建时间
    pub modified: String,       // 修改时间
    pub accessed: String,       // 访问时间
    pub is_empty: bool,         // 是否为空（目录）
}

// 前端需要的树节点格式
#[derive(Serialize, Debug)]
pub struct TreeNode {
    pub key: String,
    pub data: NodeData,
    pub children: Option<Vec<TreeNode>>,
}

// 目录统计信息
#[derive(Serialize, Debug)]
pub struct DirectoryStats {
    pub total_dirs: u64,
    pub total_files: u64,
    pub total_size: u64,
    pub is_empty: bool,
    pub tree: Vec<TreeNode>,
}

// 系统时间转格式化字符串
pub fn system_time_to_formatted(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// 格式化文件大小
fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    let decimals = if size < 10.0 {
        2
    } else if size < 100.0 {
        1
    } else {
        0
    };
    format!("{size:.decimals$}{}", UNITS[unit_index])
}

// 获取文件类型
fn get_file_type(path: &str, is_dir: bool) -> String {
    if is_dir {
        return "Folder".to_string();
    }

    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext.to_lowercase().as_str() {
            "app" | "exe" => "Application",
            "txt" | "md" | "log" => "Text",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" => "Image",
            "mp3" | "wav" | "flac" => "Audio",
            "mp4" | "avi" | "mov" | "mkv" => "Video",
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => "Document",
            "zip" | "rar" | "7z" | "tar" | "gz" => "Archive",
            _ => "File",
        })
        .unwrap_or("File")
        .to_string()
}

// 递归创建树节点
fn create_tree_node(path: &Path, recursive: bool) -> Result<TreeNode> {
    let metadata = fs::metadata(path)?;
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let full_path = path.to_string_lossy().to_string();
    let is_dir = metadata.is_dir();

    // 计算大小
    let size_bytes = if is_dir {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .fold(0, |acc, m| acc + m.len())
    } else {
        metadata.len()
    };

    // 获取时间信息
    let created = system_time_to_formatted(metadata.created()?);
    let modified = system_time_to_formatted(metadata.modified()?);
    let accessed = system_time_to_formatted(metadata.accessed()?);

    // 创建子节点
    let children = if recursive && is_dir {
        let mut child_nodes = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            let child_node = create_tree_node(&child_path, recursive)?;
            child_nodes.push(child_node);
        }

        // 对子节点进行排序：先目录后文件，目录按名称排序，文件按类型、扩展名和名称排序
        child_nodes.sort_by(|a, b| {
            // 比较类型：目录在前，文件在后
            match (&a.data.entry_type, &b.data.entry_type) {
                // 都是目录 - 按名称排序
                (EntryType::Directory, EntryType::Directory) => {
                    a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase())
                }
                // 都是文件 - 按类型、扩展名和名称排序
                (EntryType::File, EntryType::File) => {
                    // 1. 先按文件类型排序
                    let file_type_cmp = a.data.file_type.cmp(&b.data.file_type);
                    if file_type_cmp != std::cmp::Ordering::Equal {
                        return file_type_cmp;
                    }

                    // 2. 相同文件类型下，按文件扩展名排序
                    let ext_a = Path::new(&a.data.path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();

                    let ext_b = Path::new(&b.data.path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();

                    let ext_cmp = ext_a.cmp(&ext_b);
                    if ext_cmp != std::cmp::Ordering::Equal {
                        return ext_cmp;
                    }

                    // 3. 相同扩展名下，按名称排序
                    a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase())
                }
                // a是目录，b是文件 - 目录在前
                (EntryType::Directory, EntryType::File) => std::cmp::Ordering::Less,
                // a是文件，b是目录 - 文件在后
                (EntryType::File, EntryType::Directory) => std::cmp::Ordering::Greater,
            }
        });

        Some(child_nodes)
    } else {
        None
    };

    // 检查目录是否为空
    let is_empty = is_dir && children.as_ref().map_or(true, |c| c.is_empty());

    Ok(TreeNode {
        key: full_path.clone(),
        data: NodeData {
            name,
            path: full_path.clone(),
            size: size_bytes,
            formatted_size: format_size(size_bytes),
            entry_type: if is_dir {
                EntryType::Directory
            } else {
                EntryType::File
            },
            file_type: get_file_type(&full_path, is_dir),
            created,
            modified,
            accessed,
            is_empty,
        },
        children,
    })
}

// 获取目录信息（递归） - 内部实现
fn inner_get_directory_info(path: &Path, recursive: bool) -> Result<DirectoryStats> {
    let mut total_dirs = 0;
    let mut total_files = 0;
    let mut total_size = 0;

    if !path.exists() {
        return Err(anyhow::anyhow!("Directory does not exist"));
    }

    // 创建树形结构
    let mut tree = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        if metadata.is_dir() {
            total_dirs += 1;
        } else {
            total_files += 1;
        }

        let size = if metadata.is_dir() {
            WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .filter(|m| m.is_file())
                .fold(0, |acc, m| acc + m.len())
        } else {
            metadata.len()
        };

        total_size += size;

        let node = create_tree_node(&path, recursive)?;
        tree.push(node);
    }

    // 对根节点的直接子项进行排序
    tree.sort_by(|a, b| {
        // 比较类型：目录在前，文件在后
        match (&a.data.entry_type, &b.data.entry_type) {
            // 都是目录 - 按名称排序
            (EntryType::Directory, EntryType::Directory) => {
                a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase())
            }
            // 都是文件 - 按类型、扩展名和名称排序
            (EntryType::File, EntryType::File) => {
                // 1. 先按文件类型排序
                let file_type_cmp = a.data.file_type.cmp(&b.data.file_type);
                if file_type_cmp != std::cmp::Ordering::Equal {
                    return file_type_cmp;
                }

                // 2. 相同文件类型下，按文件扩展名排序
                let ext_a = Path::new(&a.data.path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                let ext_b = Path::new(&b.data.path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                let ext_cmp = ext_a.cmp(&ext_b);
                if ext_cmp != std::cmp::Ordering::Equal {
                    return ext_cmp;
                }

                // 3. 相同扩展名下，按名称排序
                a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase())
            }
            // a是目录，b是文件 - 目录在前
            (EntryType::Directory, EntryType::File) => std::cmp::Ordering::Less,
            // a是文件，b是目录 - 文件在后
            (EntryType::File, EntryType::Directory) => std::cmp::Ordering::Greater,
        }
    });

    Ok(DirectoryStats {
        total_dirs,
        total_files,
        total_size,
        is_empty: tree.is_empty(),
        tree,
    })
}

// Tauri 命令 - 获取目录信息
#[tauri::command]
pub fn get_directory_info(path: String, recursive: bool) -> Result<DirectoryStats, String> {
    inner_get_directory_info(Path::new(&path), recursive).map_err(|e| e.to_string())
}

// Tauri 命令 - 创建目录或文件
#[tauri::command]
pub fn create_entry(path: String, is_directory: bool) -> Result<(), String> {
    let path = Path::new(&path);
    if is_directory {
        fs::create_dir_all(path).map_err(|e| format!("Failed to create directory: {}", e))
    } else {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
        }
        fs::File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))
            .map(|_| ())
    }
}

// Tauri 命令 - 批量删除
#[tauri::command]
pub fn delete_entries(paths: Vec<String>) -> Result<(), String> {
    // 1. 路径规范化
    let unique_paths: HashSet<PathBuf> = paths
        .into_iter()
        .map(|p| {
            PathBuf::from(p.clone())
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(p))
        })
        .collect();

    // 2. 按路径深度降序排序（最深路径在前）
    let mut sorted_paths: Vec<PathBuf> = unique_paths.into_iter().collect();
    sorted_paths.sort_by(|a, b| b.components().count().cmp(&a.components().count()));

    // 3. 安全删除
    for path in sorted_paths {
        if !path.exists() {
            continue; // 路径已不存在，跳过
        }

        if path.is_dir() {
            // 安全删除目录
            if let Err(e) = fs::remove_dir_all(&path) {
                // 忽略"目录不存在"错误
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(format!("删除目录失败: {} - {}", path.display(), e));
                }
            }
        } else if path.is_file() {
            // 安全删除文件
            if let Err(e) = fs::remove_file(&path) {
                // 忽略"文件不存在"错误
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(format!("删除文件失败: {} - {}", path.display(), e));
                }
            }
        }
    }

    Ok(())
}

//只删除空的文件夹
#[tauri::command]
pub fn delete_empty_directories(paths: Vec<String>) -> Result<(), String> {
    // 创建路径列表并排序（确保最深层的路径先处理）
    let mut sorted_paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();

    // 按路径深度降序排序（最深的在前）
    sorted_paths.sort_by(|a, b| b.components().count().cmp(&a.components().count()));

    for path in sorted_paths {
        // 检查路径是否存在
        if !path.exists() {
            continue; // 目录已不存在，跳过
        }

        // 检查是否是目录
        if !path.is_dir() {
            return Err(format!("路径不是目录: {}", path.display()));
        }

        // 检查目录是否为空
        if is_empty(&path)? {
            // 安全删除空目录
            fs::remove_dir(&path).map_err(|e| {
                // 处理目录不存在的情况
                if e.kind() == io::ErrorKind::NotFound {
                    format!("目录已不存在: {}", path.display())
                } else {
                    format!("删除目录失败: {} - {}", path.display(), e)
                }
            })?;
        } else {
            return Err(format!("目录非空: {}", path.display()));
        }
    }
    Ok(())
}

/// 检查目录是否为空（不包含任何文件或子目录）
fn is_empty(path: &Path) -> Result<bool, String> {
    let mut entries =
        fs::read_dir(path).map_err(|e| format!("无法读取目录内容: {} - {}", path.display(), e))?;

    Ok(entries.next().is_none())
}

// Tauri 命令 - 批量重命名
#[tauri::command]
pub fn rename_entries(rename_map: HashMap<String, String>) -> Result<(), String> {
    for (old_path, new_path) in rename_map {
        let old_path = Path::new(&old_path);
        let new_path = Path::new(&new_path);

        if let Some(parent) = new_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!(
                        "Failed to create parent directory: {} - {}",
                        parent.display(),
                        e
                    )
                })?;
            }
        }

        fs::rename(old_path, new_path).map_err(|e| {
            format!(
                "Failed to rename {} to {} - {}",
                old_path.display(),
                new_path.display(),
                e
            )
        })?;
    }
    Ok(())
}

// Tauri 命令 - 打开文件所在位置
#[tauri::command]
pub fn open_entry_location(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let (path_to_open, is_file) = if path_buf.is_file() {
        // 对于文件，打开其父目录
        match path_buf.parent() {
            Some(parent) => (parent.to_path_buf(), true),
            None => {
                return Err(format!("File has no parent directory: {}", path));
            }
        }
    } else {
        // 对于目录，直接打开该目录
        (path_buf, false)
    };

    let os = std::env::consts::OS;
    let result = match os {
        "windows" => {
            if is_file {
                // Windows 中选中文件
                Command::new("explorer")
                    .arg("/select,")
                    .arg(&path) // 使用原始路径（包含文件名）
                    .spawn()
            } else {
                // 直接打开目录
                Command::new("explorer").arg(path_to_open).spawn()
            }
        }
        "macos" => {
            if is_file {
                // macOS 中定位到文件
                Command::new("open")
                    .arg("-R")
                    .arg(&path) // 使用原始路径（包含文件名）
                    .spawn()
            } else {
                // 直接打开目录
                Command::new("open").arg(path_to_open).spawn()
            }
        }
        _ => {
            // Linux 和其他类 Unix 系统
            let mut cmd = Command::new("xdg-open");
            if is_file {
                // 尝试打开父目录
                cmd.arg(path_to_open);
            } else {
                cmd.arg(path_to_open);
            }
            cmd.spawn()
        }
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open location: {}", e)),
    }
}

/// Tauri 命令 - 批量移动文件/目录到新目录
#[tauri::command]
pub fn move_entries(
    sources: Vec<String>,
    target_dir: String,
    overwrite: bool,
) -> Result<(), String> {
    let target_path = Path::new(&target_dir);

    // 确保目标目录存在
    if !target_path.exists() {
        fs::create_dir_all(target_path)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    } else if !target_path.is_dir() {
        return Err("Target path is not a directory".to_string());
    }

    // 处理每个源路径
    for src in sources {
        let source_path = Path::new(&src);
        if !source_path.exists() {
            return Err(format!("Source path does not exist: {}", src));
        }

        // 获取源文件名/目录名
        let name = source_path
            .file_name()
            .ok_or_else(|| format!("Invalid source path: {}", src))?
            .to_str()
            .ok_or("Invalid UTF-8 in file name")?;

        let dest_path = target_path.join(name);

        // 处理目标路径已存在的情况
        if dest_path.exists() {
            if overwrite {
                // 递归删除已存在的目标
                if dest_path.is_dir() {
                    fs::remove_dir_all(&dest_path)
                        .map_err(|e| format!("Failed to remove existing directory: {}", e))?;
                } else {
                    fs::remove_file(&dest_path)
                        .map_err(|e| format!("Failed to remove existing file: {}", e))?;
                }
            } else {
                return Err(format!(
                    "Target already exists and overwrite is disabled: {}",
                    dest_path.display()
                ));
            }
        }

        // 执行移动操作
        fs::rename(&source_path, &dest_path)
            .map_err(|e| format!("Failed to move {} to {}: {}", src, dest_path.display(), e))?;
    }

    Ok(())
}
