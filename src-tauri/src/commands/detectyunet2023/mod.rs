use opencv::{ core::{ Mat, Rect, Size, Vector }, imgcodecs, imgproc, objdetect::FaceDetectorYN, prelude::* };
use serde::{ Deserialize, Serialize };

// 人脸检测结果结构
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FaceDetection {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    landmarks: Vec<Landmark>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Landmark {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FaceFeatures {
    features: Vec<f32>,
}

const FACE_YUNET_DATA: &[u8] = include_bytes!("../../../models/face_detection_yunet_2023mar.onnx");

#[tauri::command]
pub fn detect_faces(image_data: Vec<u8>) -> Result<Vec<FaceDetection>, String> {
    // 1. 加载图像
    let image = imgcodecs::imdecode(&Mat::from_slice(&image_data).map_err(|e| e.to_string())?, imgcodecs::IMREAD_COLOR).map_err(|e| format!("图像解码失败: {}", e))?;

    let img_width = image.cols();
    let img_height = image.rows();
    println!("输入图像尺寸: {}x{}", img_width, img_height);

    // 2. 将静态字节切片转换为OpenCV的Vector<u8>
    let model_data = Vector::from_iter(FACE_YUNET_DATA.iter().cloned());
    let config_data = Vector::new(); // 配置文件为空，因为ONNX模型不需要额外配置

    // 3. 从内存初始化Yunet检测器
    let mut detector = FaceDetectorYN::create_1(
        "onnx", // 框架名称
        &model_data, // 模型数据
        &config_data, // 配置数据（空）
        Size::new(img_width, img_height),
        0.9, // 分数阈值
        0.3, // NMS阈值
        5000, // 最大检测数
        0, // 后端ID
        0 // 目标ID
    ).map_err(|e| format!("初始化检测器失败: {}", e))?;

    // 4. 执行检测并获取结果矩阵
    let mut results = Mat::default();
    detector.detect(&image, &mut results).map_err(|e| format!("检测失败: {}", e))?;

    // 5. 解析结果矩阵
    let mut detections = Vec::new();

    // 结果矩阵的每一行代表一个检测到的人脸，每行有15个元素：
    // [x, y, w, h, x_re, y_re, x_le, y_le, x_nt, y_nt, x_rcm, y_rcm, x_lcm, y_lcm, score]
    for i in 0..results.rows() {
        // 使用Mat::at_2d访问元素并解引用
        let x = *results.at_2d::<f32>(i, 0).map_err(|e| e.to_string())? as i32;
        let y = *results.at_2d::<f32>(i, 1).map_err(|e| e.to_string())? as i32;
        let width = *results.at_2d::<f32>(i, 2).map_err(|e| e.to_string())? as i32;
        let height = *results.at_2d::<f32>(i, 3).map_err(|e| e.to_string())? as i32;

        // 解析5个关键点 (右眼、左眼、鼻尖、右嘴角、左嘴角)
        let mut landmarks = Vec::new();
        for j in 0..5 {
            let offset = 4 + j * 2;
            landmarks.push(Landmark {
                x: *results.at_2d::<f32>(i, offset).map_err(|e| e.to_string())?,
                y: *results.at_2d::<f32>(i, offset + 1).map_err(|e| e.to_string())?,
            });
        }

        detections.push(FaceDetection {
            x,
            y,
            width,
            height,
            landmarks,
        });
    }

    println!("检测到的人脸数量: {}", detections.len());
    Ok(detections)
}

// 特征提取函数
#[tauri::command]
pub fn extract_features(image_data: Vec<u8>, face: FaceDetection) -> Result<FaceFeatures, String> {
    let image = imgcodecs::imdecode(&Mat::from_slice(&image_data).map_err(|e| e.to_string())?, imgcodecs::IMREAD_COLOR).map_err(|e| e.to_string())?;

    let roi = Rect::new(face.x, face.y, face.width, face.height);
    let face_roi = Mat::roi(&image, roi).map_err(|e| e.to_string())?;

    let mut resized_face = Mat::default();
    imgproc::resize(&face_roi, &mut resized_face, Size::new(100, 100), 0.0, 0.0, imgproc::INTER_LINEAR).map_err(|e| e.to_string())?;

    let mut gray_face = Mat::default();
    imgproc
        ::cvt_color(
            &resized_face,
            &mut gray_face,
            imgproc::COLOR_BGR2GRAY,
            0, // 目标通道数（0表示自动）
            opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT // 使用正确的算法提示
        )
        .map_err(|e| e.to_string())?;

    let mut features = Vec::new();
    for i in 0..gray_face.rows() {
        for j in 0..gray_face.cols() {
            let pixel = *gray_face.at_2d::<u8>(i, j).map_err(|e| e.to_string())?;
            features.push((pixel as f32) / 255.0);
        }
    }

    Ok(FaceFeatures { features })
}

// 特征比较函数保持不变
#[tauri::command]
pub fn compare_features(features1: Vec<f32>, features2: Vec<f32>) -> f32 {
    if features1.len() != features2.len() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;

    for i in 0..features1.len() {
        dot_product += features1[i] * features2[i];
        norm1 += features1[i].powi(2);
        norm2 += features2[i].powi(2);
    }

    let norm1 = norm1.sqrt();
    let norm2 = norm2.sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot_product / (norm1 * norm2)
    }
}
