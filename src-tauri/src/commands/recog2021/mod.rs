use std::{ collections::HashMap, ffi::OsStr, fs::{ self, File }, io::{ self, Read, Write }, path::{ Path, PathBuf }, sync::{ Arc, RwLock } };
use tauri::{ Manager, State, Window };
use opencv::{ core, dnn, imgcodecs, imgproc, objdetect, prelude::*, videoio };
use base64::Engine as _;
use serde::{ Deserialize, Serialize };
use tauri::Emitter;
use tauri::AppHandle;
// 嵌入模型文件到可执行文件中
const FACE_NET_DATA: &[u8] = include_bytes!("../../../models/face_recognition_sface_2021dec.onnx");
const DETECTOR_DATA: &[u8] = include_bytes!("../../../models/haarcascade_frontalface_default.xml");

// 人脸数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    name: String,
    features: Vec<f32>,
    image_path: String,
}

// 应用状态
pub struct AppRecog2021State {
    pub face_db: Arc<RwLock<HashMap<String, FaceData>>>,
    pub app_dir: PathBuf,
    pub db_file_path: PathBuf,
    pub pictures_dir: PathBuf,
}

/// 初始化人脸识别相关资源和状态
pub fn init_face_recognition(app: &AppHandle) -> Result<(), String> {
    // 初始化应用数据目录
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| "无法获取应用数据目录")?;
    std::fs::create_dir_all(&app_dir).map_err(|e| format!("创建应用目录失败: {}", e))?;

    // 创建图片存储目录
    let pictures_dir = app_dir.join("pictures");
    std::fs::create_dir_all(&pictures_dir).map_err(|e| format!("创建图片目录失败: {}", e))?;

    // 数据库文件路径
    let db_file_path = app_dir.join("face_database.json");

    // 从文件加载人脸数据库
    let face_db = match load_face_db(&db_file_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("加载人脸数据库失败: {}", e);
            HashMap::new()
        }
    };

    // 创建并管理状态
    let state = AppRecog2021State {
        face_db: Arc::new(std::sync::RwLock::new(face_db)),
        app_dir: app_dir.clone(),
        db_file_path,
        pictures_dir,
    };
    app.manage(state);

    Ok(())
}

// 从JSON文件加载人脸数据库
pub fn load_face_db(db_file_path: &Path) -> io::Result<HashMap<String, FaceData>> {
    if !db_file_path.exists() {
        return Ok(HashMap::new());
    }

    let mut file = File::open(db_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let face_db: HashMap<String, FaceData> = serde_json::from_str(&contents)?;
    Ok(face_db)
}

// 将人脸数据库保存到JSON文件
pub fn save_face_db(face_db: &HashMap<String, FaceData>, db_file_path: &Path) -> io::Result<()> {
    let json_data = serde_json::to_string_pretty(face_db)?;
    let mut file = File::create(db_file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

// 人脸特征比较函数
pub fn compare_faces(features1: &[f32], features2: &[f32]) -> f32 {
    let dot_product: f32 = features1
        .iter()
        .zip(features2.iter())
        .map(|(&a, &b)| a * b)
        .sum();
    let norm1 = features1
        .iter()
        .map(|&x| x * x)
        .sum::<f32>()
        .sqrt();
    let norm2 = features2
        .iter()
        .map(|&x| x * x)
        .sum::<f32>()
        .sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot_product / (norm1 * norm2)
    }
}

// 从内存加载级联分类器
pub fn load_cascade_classifier_from_memory() -> Result<objdetect::CascadeClassifier, String> {
    // 创建临时文件用于加载分类器
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("haarcascade_frontalface_default.xml");

    // 如果文件不存在或大小不匹配，写入新文件
    if
        !temp_file.exists() ||
        fs
            ::metadata(&temp_file)
            .map(|m| m.len() as usize)
            .unwrap_or(0) != DETECTOR_DATA.len()
    {
        fs::write(&temp_file, DETECTOR_DATA).map_err(|e| format!("写入临时分类器文件失败: {}", e))?;
    }

    // 从文件路径加载分类器
    objdetect::CascadeClassifier::new(&temp_file.to_string_lossy()).map_err(|e| format!("加载人脸检测器失败: {}", e))
}

#[tauri::command]
pub fn add_face(state: State<'_, AppRecog2021State>, name: String, image_data: Vec<u8>, file_name: String) -> Result<String, String> {
    println!("添加人脸: {}", name);

    // 创建人脸存储目录
    let faces_dir = state.app_dir.join("faces");
    if !faces_dir.exists() {
        std::fs::create_dir_all(&faces_dir).map_err(|e| format!("创建人脸目录失败: {}", e))?;
    }

    // 生成唯一文件名
    let file_ext = Path::new(&file_name).extension().and_then(OsStr::to_str).unwrap_or("jpg");
    let unique_file_name = format!("{}_{}.{}", name.replace(" ", "_"), chrono::Utc::now().format("%Y%m%d%H%M%S"), file_ext);
    let image_path = faces_dir.join(&unique_file_name);

    // 保存图片到文件系统
    std::fs::write(&image_path, &image_data).map_err(|e| format!("保存图片失败: {}", e))?;

    // 显式指定 Vector 的类型为 u8
    let vec_data = core::Vector::<u8>::from_iter(image_data.iter().cloned());

    // 读取图片
    let img = imgcodecs::imdecode(&vec_data, imgcodecs::IMREAD_COLOR).map_err(|e| format!("解码图片失败: {}", e))?;

    // 加载人脸检测器
    let mut detector = load_cascade_classifier_from_memory()?;

    // 转换为灰度图用于人脸检测
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0, core::AlgorithmHint::ALGO_HINT_DEFAULT).map_err(|e| format!("转换为灰度图失败: {}", e))?;

    // 检测人脸 - 使用 core::Vector<core::Rect> 替代 VectorOfRect
    let mut faces = core::Vector::<core::Rect>::new();
    detector.detect_multi_scale(&gray, &mut faces, 1.1, 3, 0, core::Size::new(30, 30), core::Size::new(0, 0)).map_err(|e| format!("检测人脸失败: {}", e))?;

    // 检查是否检测到人脸
    if faces.is_empty() {
        // 删除保存的图片
        let _ = std::fs::remove_file(&image_path);
        return Err("在图片中未检测到人脸".to_string());
    }

    // 如果有多个人脸，只取第一个
    if faces.len() > 1 {
        println!("警告: 图片中检测到多个人脸，仅使用第一个");
    }

    let face_rect = faces.get(0).unwrap();

    // 提取人脸区域
    let face_roi = Mat::roi(&img, face_rect).map_err(|e| format!("提取人脸区域失败: {}", e))?;

    // 从内存加载人脸识别模型
    // 将静态字节切片转换为OpenCV的Vector<u8>
    let face_net_vec = core::Vector::from_iter(FACE_NET_DATA.iter().cloned());
    // 从内存加载人脸识别模型
    let mut face_net = dnn
        ::read_net_from_onnx_buffer(&face_net_vec) // 传入Vector<u8>的引用
        .map_err(|e| format!("加载人脸识别模型失败: {}", e))?;

    // 准备输入数据
    let blob = dnn::blob_from_image(&face_roi, 1.0, core::Size::new(112, 112), core::Scalar::from((0.0, 0.0, 0.0)), true, false, core::CV_32F).map_err(|e| format!("创建输入数据失败: {}", e))?;

    // 设置网络输入
    face_net.set_input(&blob, "", 1.0, core::Scalar::all(0.0)).map_err(|e| format!("设置网络输入失败: {}", e))?;

    // 获取模型的输出层名称
    let output_names = {
        let layer_names = face_net.get_unconnected_out_layers_names().unwrap();
        let mut names = core::Vector::<String>::new();
        for i in 0..layer_names.len() {
            names.push(&layer_names.get(i).unwrap().to_string());
        }
        names
    };

    // 运行模型前向传播
    let mut output = core::Vector::<Mat>::new();
    face_net.forward(&mut output, &output_names).map_err(|e| format!("运行模型失败: {}", e))?;

    // 提取特征向量
    let features = output
        .get(0)
        .map_err(|e| format!("获取特征向量失败: {}", e))?
        .to_vec_2d::<f32>()
        .map_err(|e| format!("转换特征向量失败: {}", e))?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    // 保存到人脸数据库
    let mut face_db = state.face_db.write().unwrap();
    face_db.insert(name.clone(), FaceData {
        name,
        features,
        image_path: image_path.to_string_lossy().into_owned(),
    });

    // 保存数据库到文件
    if let Err(e) = save_face_db(&face_db, &state.db_file_path) {
        eprintln!("保存人脸数据库失败: {}", e);
        // 即使保存失败，也返回成功，因为内存中已添加
    }

    Ok("人脸添加成功".to_string())
}

#[tauri::command]
pub fn delete_face(state: State<'_, AppRecog2021State>, name: String) -> Result<String, String> {
    let mut face_db = state.face_db.write().unwrap();

    // 检查人脸是否存在并获取其图片路径
    if let Some(face_data) = face_db.get(&name) {
        let image_path = PathBuf::from(&face_data.image_path);

        // 删除图片文件
        if image_path.exists() {
            if let Err(e) = std::fs::remove_file(&image_path) {
                eprintln!("删除图片失败: {}", e);
                // 即使删除图片失败，仍继续删除数据库记录
            }
        }
    }

    // 从数据库中删除人脸记录
    face_db.remove(&name);

    // 保存数据库到文件
    if let Err(e) = save_face_db(&face_db, &state.db_file_path) {
        eprintln!("保存人脸数据库失败: {}", e);
        // 即使保存失败，也返回成功，因为内存中已删除
    }

    Ok("人脸删除成功".to_string())
}

#[tauri::command]
pub fn get_all_faces(state: State<'_, AppRecog2021State>) -> Result<Vec<FaceData>, String> {
    let face_db = state.face_db.read().unwrap();
    Ok(face_db.values().cloned().collect())
}

#[tauri::command]
pub fn start_recognition(state: State<'_, AppRecog2021State>, window: Window) -> Result<String, String> {
    // 使用 Arc 共享人脸数据库
    let face_db = Arc::clone(&state.face_db);
    let window = Arc::new(std::sync::Mutex::new(window));

    std::thread::spawn(move || {
        // 加载人脸检测器
        let mut detector = match load_cascade_classifier_from_memory() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("加载检测器失败: {}", e);
                return;
            }
        };

        // 将静态字节切片转换为OpenCV的Vector<u8>
        let face_net_vec = core::Vector::from_iter(FACE_NET_DATA.iter().cloned());
        // 从内存加载人脸识别模型
        let mut face_net = match dnn::read_net_from_onnx_buffer(&face_net_vec) {
            // 传入Vector<u8>的引用
            Ok(net) => net,
            Err(e) => {
                eprintln!("加载人脸识别模型失败: {}", e);
                return;
            }
        };
        // 打开摄像头
        let mut camera = match videoio::VideoCapture::new(0, videoio::CAP_ANY) {
            Ok(cam) => cam,
            Err(e) => {
                eprintln!("打开摄像头失败: {}", e);
                return;
            }
        };

        if !camera.is_opened().unwrap_or(false) {
            eprintln!("摄像头未打开");
            return;
        }

        let mut frame = Mat::default();
        let mut result_frame = Mat::default();

        loop {
            // 读取一帧
            if let Err(e) = camera.read(&mut frame) {
                eprintln!("读取帧失败: {}", e);
                break;
            }

            if frame.empty() {
                eprintln!("帧为空");
                break;
            }

            // 克隆帧用于结果显示
            if let Err(e) = frame.copy_to(&mut result_frame) {
                eprintln!("复制帧失败: {}", e);
                break;
            }

            // 转换为灰度图
            let mut gray = Mat::default();
            if let Err(e) = imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0, core::AlgorithmHint::ALGO_HINT_DEFAULT) {
                eprintln!("转换为灰度图失败: {}", e);
                continue;
            }

            // 检测人脸 - 使用 core::Vector<core::Rect> 替代 VectorOfRect
            let mut faces = core::Vector::<core::Rect>::new();
            if let Err(e) = detector.detect_multi_scale(&gray, &mut faces, 1.1, 3, 0, core::Size::new(30, 30), core::Size::new(0, 0)) {
                eprintln!("检测人脸失败: {}", e);
                continue;
            }

            // 对每个检测到的人脸进行识别
            for face_rect in faces.iter() {
                // 提取人脸区域
                let face_roi = match Mat::roi(&frame, face_rect) {
                    Ok(roi) => roi,
                    Err(e) => {
                        eprintln!("提取人脸区域失败: {}", e);
                        continue;
                    }
                };

                // 提取特征
                let blob = match dnn::blob_from_image(&face_roi, 1.0, core::Size::new(112, 112), core::Scalar::from((0.0, 0.0, 0.0)), true, false, core::CV_32F) {
                    Ok(blob) => blob,
                    Err(e) => {
                        eprintln!("创建输入数据失败: {}", e);
                        continue;
                    }
                };

                if let Err(e) = face_net.set_input(&blob, "", 1.0, core::Scalar::all(0.0)) {
                    eprintln!("设置网络输入失败: {}", e);
                    continue;
                }

                // 获取模型的输出层名称
                let output_names = {
                    let layer_names = face_net.get_unconnected_out_layers_names().unwrap();
                    let mut names = core::Vector::<String>::new();
                    for i in 0..layer_names.len() {
                        names.push(&layer_names.get(i).unwrap().to_string());
                    }
                    names
                };

                // 运行模型前向传播
                let mut output = core::Vector::<Mat>::new();
                if let Err(e) = face_net.forward(&mut output, &output_names) {
                    eprintln!("运行模型失败: {}", e);
                    continue;
                }

                if output.is_empty() {
                    continue;
                }

                let features = match output.get(0) {
                    Ok(mat) => mat.to_vec_2d::<f32>().unwrap().into_iter().flatten().collect::<Vec<_>>(),
                    Err(e) => {
                        eprintln!("获取特征向量失败: {}", e);
                        continue;
                    }
                };

                // 在数据库中查找匹配
                let mut best_match = None;
                let mut best_similarity = 0.6; // 阈值

                let face_db_guard = match face_db.read() {
                    Ok(guard) => guard,
                    Err(e) => {
                        eprintln!("锁定人脸数据库失败: {}", e);
                        continue;
                    }
                };

                for face_data in face_db_guard.values() {
                    let similarity = compare_faces(&features, &face_data.features);
                    if similarity > best_similarity {
                        best_similarity = similarity;
                        best_match = Some(face_data);
                    }
                }

                // 绘制人脸框和信息
                let color = if best_match.is_some() {
                    core::Scalar::new(0.0, 255.0, 0.0, 0.0) // 绿色：识别成功
                } else {
                    core::Scalar::new(0.0, 0.0, 255.0, 0.0) // 红色：未识别
                };

                if let Err(e) = imgproc::rectangle(&mut result_frame, face_rect, color, 2, imgproc::LINE_8, 0) {
                    eprintln!("绘制矩形失败: {}", e);
                }

                if let Some(face_data) = best_match {
                    let label = format!("{}: {:.2}", face_data.name, best_similarity);
                    let pos = core::Point::new(face_rect.x, face_rect.y - 10);
                    if let Err(e) = imgproc::put_text(&mut result_frame, &label, pos, imgproc::FONT_HERSHEY_SIMPLEX, 0.7, color, 2, imgproc::LINE_AA, false) {
                        eprintln!("添加文本失败: {}", e);
                    }
                }
            }

            // 将处理后的帧发送到前端
            let mut buf = core::Vector::<u8>::new();
            let params = core::Vector::<i32>::new();

            if let Err(e) = imgcodecs::imencode(".jpg", &result_frame, &mut buf, &params) {
                eprintln!("编码图像失败: {}", e);
                continue;
            }

            let img_str = base64::engine::general_purpose::STANDARD.encode(buf.as_slice());
            let window_guard = match window.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    eprintln!("锁定窗口失败: {}", e);
                    continue;
                }
            };

            if let Err(e) = window_guard.emit("video-frame", img_str) {
                eprintln!("发送视频帧失败: {}", e);
                break;
            }

            // 短暂休眠，避免CPU占用过高
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    });

    Ok("人脸识别已启动".to_string())
}

#[tauri::command]
pub fn get_face_image(path: String) -> Result<String, String> {
    // 读取图片文件
    let image_data = std::fs::read(&path).map_err(|e| format!("读取图片失败: {}", e))?;

    // 转换为Base64
    let base64_str = base64::engine::general_purpose::STANDARD.encode(&image_data);
    Ok(base64_str)
}

// 新增：保存拍摄的帧
#[tauri::command]
pub fn save_captured_frame(state: State<'_, AppRecog2021State>, base64_data: String, file_name: String) -> Result<String, String> {
    // 创建图片存储目录
    if !state.pictures_dir.exists() {
        std::fs::create_dir_all(&state.pictures_dir).map_err(|e| format!("创建图片目录失败: {}", e))?;
    }

    // 解码Base64数据
    let image_data = base64::engine::general_purpose::STANDARD.decode(&base64_data).map_err(|e| format!("Base64解码失败: {}", e))?;

    // 保存到文件
    let file_path = state.pictures_dir.join(&file_name);
    std::fs::write(&file_path, &image_data).map_err(|e| format!("保存图片失败: {}", e))?;

    println!("图片保存成功: {}", file_path.display());
    Ok("图片保存成功".to_string())
}
