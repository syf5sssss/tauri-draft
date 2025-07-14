use anyhow::{ anyhow, Result };
use image::RgbImage;
use once_cell::sync::Lazy;
use rust_faces::{ BlazeFaceParams, FaceDetection, FaceDetector, FaceDetectorBuilder, InferParams, Provider, ToArray3 };
use serde::{ Deserialize, Serialize };
use std::path::Path;

// 新增：人脸位置结构体（供前端绘制边框用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FaceRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// 人脸数据结构 , Encode, Decode
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FaceData {
    pub id: String,
    pub name: String,
    pub feature: Vec<f32>,
}

// 全局人脸检测器（已存在）
static DETECTOR: Lazy<Box<dyn FaceDetector>> = Lazy::new(|| {
    let model_path = "models/blazefaces-640.onnx".to_string();

    FaceDetectorBuilder::new(FaceDetection::BlazeFace640(BlazeFaceParams::default()))
        .from_file(model_path)
        .infer_params(InferParams {
            provider: Provider::OrtCpu,
            intra_threads: Some(4),
            ..Default::default()
        })
        .build()
        .expect("Failed to initialize face detector")
});

/// 初始化检测器和数据库（已存在）
pub fn init() -> Result<()> {
    if !Path::new("models/blazefaces-640.onnx").exists() {
        return Err(anyhow!("Model file not found. Please download and place in models/ directory"));
    }
    let _ = &*DETECTOR;
    Ok(())
}

/// 新增：检测人脸位置（返回人脸边框坐标）
pub fn detect_face_position(image: RgbImage) -> Result<Option<FaceRect>> {
    // 转换图像格式为检测器所需的数组格式
    let rgb_array = image.into_array3();
    let faces = DETECTOR.detect(rgb_array.view().into_dyn())?;

    // 取第一个检测到的人脸
    if let Some(face) = faces.first() {
        let rect = face.rect;
        Ok(
            Some(FaceRect {
                x: rect.x as f64,
                y: rect.y as f64,
                width: rect.width as f64,
                height: rect.height as f64,
            })
        )
    } else {
        Ok(None)
    }
}
