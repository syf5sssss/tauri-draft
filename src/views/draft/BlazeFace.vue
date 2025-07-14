<template>
    <div class="container">
        <h1>人脸检测系统</h1>

        <!-- 摄像头控制区域 -->
        <div class="section">
            <div class="camera-controls">
                <select v-model="selectedCamera" @change="switchCamera">
                    <option value="">选择摄像头</option>
                    <option v-for="camera in cameras" :value="camera.deviceId" :key="camera.deviceId">
                        {{ camera.label || `摄像头 ${camera.deviceId.substring(0, 8)}` }}
                    </option>
                </select>
                <button @click="toggleCamera">{{ isCameraActive ? '关闭摄像头' : '打开摄像头' }}</button>
                <button @click="capturePhoto" :disabled="!isCameraActive || !hasDetectedFace"
                    :class="{ 'btn-disabled': !hasDetectedFace }">
                    {{ hasDetectedFace ? '拍摄（已检测到人脸）' : '拍摄（未检测到人脸）' }}
                </button>
            </div>

            <div class="camera-preview">
                <!-- 视频容器（用于叠加人脸框） -->
                <div class="video-container relative">
                    <video ref="videoElement" autoplay playsinline
                        :style="{ display: isCameraActive ? 'block' : 'none' }" class="preview"></video>
                    <!-- 人脸框画布（叠加在视频上方） -->
                    <canvas ref="faceDetectionCanvas" :style="{
                        display: isCameraActive ? 'block' : 'none',
                        position: 'absolute',
                        top: '0',
                        left: '0',
                        pointerEvents: 'none'
                    }" class="preview-overlay"></canvas>
                </div>
                <canvas ref="canvasElement" :style="{ display: capturedImage ? 'block' : 'none' }"
                    class="preview"></canvas>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, ref as vueRef, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { documentDir } from '@tauri-apps/api/path';

// 人脸检测相关变量
const faceDetectionCanvas = vueRef(null); // 用于绘制人脸框的画布
const hasDetectedFace = ref(false); // 是否检测到人脸
let faceDetectionInterval = null; // 人脸框检测定时器

// 摄像头相关变量
const videoElement = vueRef(null);
const canvasElement = vueRef(null);
const cameras = ref([]);
const selectedCamera = ref('');
const isCameraActive = ref(false);
const capturedImage = ref('');
let mediaStream = null;

// 检测相关变量
const detectImagePath = ref('');
const previewImage = ref('');

// 初始化：加载摄像头列表和人脸数据
onMounted(async () => {
    try {
        // 加载摄像头列表
        await loadCameras();
    } catch (e) {
        console.error('初始化失败:', e);
    }
});

// 加载可用摄像头
const loadCameras = async () => {
    try {
        // 先请求摄像头权限
        const tempStream = await navigator.mediaDevices.getUserMedia({ video: true });
        // 立即停止临时流
        tempStream.getTracks().forEach(track => track.stop());

        // 枚举设备
        const devices = await navigator.mediaDevices.enumerateDevices();
        const videoDevices = devices.filter(device => device.kind === 'videoinput');
        // 过滤掉无效设备
        cameras.value = videoDevices.filter(device => device.deviceId.trim() !== '');
        // 默认选择第一个有效摄像头
        if (cameras.value.length > 0 && !selectedCamera.value) {
            selectedCamera.value = cameras.value[0].deviceId;
        }
    } catch (e) {
        console.error('获取摄像头失败:', e);
        alert('请允许摄像头权限以使用相关功能');
    }
};

// 切换摄像头状态（打开/关闭）
const toggleCamera = async () => {
    if (isCameraActive.value) {
        await stopCamera();
    } else {
        await startCamera();
    }
};

// 启动摄像头
const startCamera = async () => {
    if (!selectedCamera.value) {
        alert('请先选择一个摄像头');
        return;
    }

    try {
        mediaStream = await navigator.mediaDevices.getUserMedia({
            video: {
                deviceId: { exact: selectedCamera.value },
                width: { ideal: 1280 },
                height: { ideal: 720 }
            }
        });

        if (videoElement.value) {
            videoElement.value.srcObject = mediaStream;
            isCameraActive.value = true;

            // 等待视频加载完成后初始化人脸框画布
            await nextTick();
            initFaceDetectionCanvas();
            startFaceDetection(); // 启动实时人脸框检测
        }
    } catch (e) {
        console.error('启动摄像头失败:', e);
        alert('无法打开摄像头: ' + e.message);
    }
};

// 停止摄像头
const stopCamera = async () => {
    if (mediaStream) {
        mediaStream.getTracks().forEach(track => track.stop());
        mediaStream = null;
    }
    isCameraActive.value = false;
    stopFaceDetection(); // 停止人脸框检测
    hasDetectedFace.value = false;
};

// 切换摄像头
const switchCamera = async () => {
    const wasActive = isCameraActive.value;
    if (wasActive) {
        await stopCamera();
    }
    await startCamera();
};

// 修改initFaceDetectionCanvas函数，确保画布尺寸正确
const initFaceDetectionCanvas = () => {
    if (!videoElement.value || !faceDetectionCanvas.value) return;

    const video = videoElement.value;
    const canvas = faceDetectionCanvas.value;

    // 等待视频元数据加载完成后再设置尺寸（关键修复）
    const setCanvasSize = () => {
        // 设置画布实际尺寸与视频原始尺寸一致
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;

        // 设置画布显示尺寸与视频显示尺寸一致
        canvas.style.width = `${video.offsetWidth}px`;
        canvas.style.height = `${video.offsetHeight}px`;
    };

    // 确保视频元数据加载完成
    if (video.videoWidth > 0 && video.videoHeight > 0) {
        setCanvasSize();
    } else {
        video.addEventListener('loadedmetadata', setCanvasSize);
    }

    // 监听窗口大小变化
    window.addEventListener('resize', setCanvasSize);
};

// 启动实时人脸框检测
const startFaceDetection = () => {
    // 先停止已有定时器
    stopFaceDetection();
    console.log("打开一个人脸检测的定时器");
    // 每300ms检测一次人脸位置
    faceDetectionInterval = setInterval(async () => {
        if (!isCameraActive.value || !videoElement.value) return;

        try {
            // 捕获当前视频帧用于人脸检测
            const video = videoElement.value;
            const tempCanvas = document.createElement('canvas');
            tempCanvas.width = video.videoWidth;
            tempCanvas.height = video.videoHeight;
            const ctx = tempCanvas.getContext('2d');
            ctx.drawImage(video, 0, 0, tempCanvas.width, tempCanvas.height);

            // 转换为base64数据
            const base64Data = tempCanvas.toDataURL('image/jpeg').split(',')[1];

            // 调用Rust后端检测人脸位置
            const faceRect = await invoke('cmd_detect_face_position', {
                imageData: base64Data
            });
            // 绘制人脸框
            drawFaceRect(faceRect);

            // 更新状态（是否检测到人脸）
            hasDetectedFace.value = !!faceRect;
        } catch (e) {
            console.error('人脸位置检测失败:', e);
            hasDetectedFace.value = false;
            clearFaceRect(); // 清除人脸框
        }
    }, 1000);
};

// 停止人脸框检测
const stopFaceDetection = () => {
    if (faceDetectionInterval) {
        clearInterval(faceDetectionInterval);
        faceDetectionInterval = null;
        console.log("清空人脸检测的定时器");
    }
    clearFaceRect(); // 清除人脸框
};

// 修改drawFaceRect函数，确保坐标正确转换
const drawFaceRect = (faceRect) => {
    if (!faceRect || !faceDetectionCanvas.value || !videoElement.value) {
        clearFaceRect();
        return;
    }

    const canvas = faceDetectionCanvas.value;
    const ctx = canvas.getContext('2d');
    const video = videoElement.value;

    // 清除之前的绘制
    clearFaceRect();

    // 计算视频实际尺寸与显示尺寸的比例（关键修复）
    const scaleX = canvas.width / video.videoWidth;
    const scaleY = canvas.height / video.videoHeight;

    // 提取人脸坐标并应用缩放（关键修复）
    const x = faceRect.x * scaleX;
    const y = faceRect.y * scaleY;
    const width = faceRect.width * scaleX;
    const height = faceRect.height * scaleY;

    // 绘制蓝色边框（线宽2px，不透明）
    ctx.strokeStyle = 'rgba(0, 123, 255, 1)';
    ctx.lineWidth = 2;
    ctx.strokeRect(x, y, width, height);

    // 添加人脸检测提示文本
    ctx.fillStyle = 'rgba(0, 123, 255, 1)';
    ctx.font = '16px Arial';
    ctx.fillText('已检测到人脸', x + 5, y - 10);
};

// 清除人脸框
const clearFaceRect = () => {
    if (!faceDetectionCanvas.value) return;

    const canvas = faceDetectionCanvas.value;
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
};

// 拍摄照片
const capturePhoto = async () => {
    if (!hasDetectedFace.value) {
        alert('未检测到人脸，请调整位置后重试');
        return;
    }

    if (!isCameraActive.value || !videoElement.value || !canvasElement.value) return;

    // 设置画布尺寸与视频一致
    const video = videoElement.value;
    const canvas = canvasElement.value;
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;

    // 绘制当前视频帧到画布
    const ctx = canvas.getContext('2d');
    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

    // 保存图片数据
    capturedImage.value = canvas.toDataURL('image/jpeg');

    // 将照片保存到文件系统
    const appDir = await documentDir();
    const path = `${appDir}/capture_${Date.now()}.jpg`;

    // 转换base64为二进制数据
    const base64Data = capturedImage.value.split(',')[1];
    const binaryData = Uint8Array.from(atob(base64Data), c => c.charCodeAt(0));

    await invoke('write_file', {
        path,
        data: binaryData
    });

    // 自动将拍摄的照片用于检测
    detectImagePath.value = path;
    previewImage.value = capturedImage.value;
};

// 组件卸载时清理资源
watch(
    () => false, // 组件卸载时触发
    () => {
        if (isCameraActive.value) {
            stopCamera();
        }
        stopFaceDetection(); // 停止人脸框检测
    }
);
</script>

<style>
.container {
    max-width: 1000px;
    margin: 0 auto;
    padding: 20px;
}

.section {
    margin: 20px 0;
    padding: 15px;
    border: 1px solid #ccc;
    border-radius: 8px;
    background-color: #f9f9f9;
}

.camera-controls {
    margin-bottom: 15px;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    align-items: center;
}

.camera-preview {
    position: relative;
    min-height: 400px;
}

.video-container {
    position: relative;
    display: inline-block;
}

.preview {
    max-width: 100%;
    max-height: 400px;
    border: 1px solid #666;
    border-radius: 4px;
    background-color: #000;
}

.preview-overlay {
    position: absolute;
    top: 0;
    left: 0;
    pointerEvents: none;
}

.detection-result {
    margin-top: 10px;
    padding: 10px;
    background-color: #e8f5e9;
    border-radius: 4px;
    border-left: 4px solid #43a047;
}

.form-group {
    margin-bottom: 10px;
}

input,
button,
select {
    margin: 5px;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
}

button {
    background-color: #42b983;
    color: white;
    border: none;
    cursor: pointer;
    transition: background-color 0.2s;
}

button:hover:not(:disabled) {
    background-color: #359e75;
}

button:disabled {
    background-color: #ccc;
    cursor: not-allowed;
}

.btn-disabled {
    background-color: #ffcccc !important;
    color: #666 !important;
}

select {
    background-color: white;
}

.status-message {
    margin-top: 8px;
    padding: 6px;
    border-radius: 4px;
}

.status-message:empty {
    display: none;
}

.status-message:contains('成功') {
    background-color: #e8f5e9;
    color: #2e7d32;
}

.status-message:contains('失败') {
    background-color: #ffebee;
    color: #c62828;
}

.status-message:contains('正在') {
    background-color: #fff3e0;
    color: #f57c00;
}
</style>