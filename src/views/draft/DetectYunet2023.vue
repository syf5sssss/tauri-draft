<template>
    <div class="face-detection-container">
        <div class="camera-container">
            <video ref="videoRef" autoplay muted playsinline></video>
            <canvas ref="canvasRef" class="overlay"></canvas>
        </div>
        <div class="controls">
            <button @click="startCamera" :disabled="cameraStarted">
                <i class="fa fa-video-camera"></i> 开启摄像头
            </button>
            <button @click="captureImage" :disabled="!cameraStarted">
                <i class="fa fa-camera"></i> 拍摄照片
            </button>
            <button @click="detectFaces" :disabled="!capturedImage">
                <i class="fa fa-search"></i> 检测人脸
            </button>
            <button @click="recognizeFace" :disabled="!detectedFace">
                <i class="fa fa-user"></i> 识别人脸
            </button>
            <button @click="saveFace" :disabled="!detectedFace || !faceName">
                <i class="fa fa-save"></i> 保存人脸
            </button>
            <button @click="clearResults" :disabled="!capturedImage">
                <i class="fa fa-refresh"></i> 清除结果
            </button>
        </div>
        <div class="face-name-input">
            <label for="faceName">人脸名称:</label>
            <input type="text" id="faceName" v-model="faceName" placeholder="请输入名称" />
        </div>
        <div class="results">
            <h3>识别结果</h3>
            <div v-if="recognitionResult" class="result-item">
                <p>识别结果: {{ recognitionResult.name }}</p>
                <p>相似度: {{ recognitionResult.similarity.toFixed(2) }}</p>
            </div>
            <div v-else class="result-item empty">
                <p>暂无识别结果</p>
            </div>
        </div>
        <div class="face-database">
            <h3>人脸库</h3>
            <div class="face-items">
                <div v-for="(face, index) in faceDatabase" :key="index" class="face-item">
                    <img :src="face.image" alt="人脸照片" />
                    <p>{{ face.name }}</p>
                    <button @click="deleteFace(index)">
                        <i class="fa fa-trash"></i> 删除
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { readTextFile, writeTextFile, mkdir, exists } from '@tauri-apps/plugin-fs'
import { appDataDir, join } from '@tauri-apps/api/path'

// 响应式变量
const videoRef = ref(null)
const canvasRef = ref(null)
const cameraStarted = ref(false)
const capturedImage = ref(null)
const detectedFace = ref(null)
const faceName = ref('')
const recognitionResult = ref(null)
const faceDatabase = ref([])

// 初始化人脸库
const initializeFaceDatabase = async () => {
    try {
        const appDirPath = await appDataDir()
        const dbPath = await join(appDirPath, 'face_database.json')
        console.log(dbPath);
        // 确保目录存在
        const dirExists = await exists(appDirPath)
        if (!dirExists) {
            // 使用 mkdir 替代 createDir
            await mkdir(appDirPath, { recursive: true })
        }

        const dbExists = await exists(dbPath)
        if (dbExists) {
            const data = await readTextFile(dbPath)
            const parsedData = JSON.parse(data)
            faceDatabase.value = Array.isArray(parsedData) ? parsedData : []
        } else {
            // 创建空数据库文件
            await writeTextFile(dbPath, JSON.stringify([]))
            faceDatabase.value = []
        }
    } catch (error) {
        console.error('初始化人脸库失败:', error)
        alert(error);
        faceDatabase.value = []
    }
}

// 保存人脸库
const saveFaceDatabase = async () => {
    try {
        const appDirPath = await appDataDir()
        const dbPath = await join(appDirPath, 'face_database.json')
        await writeTextFile(dbPath, JSON.stringify(faceDatabase.value))
    } catch (error) {
        console.error('保存人脸库失败:', error)
    }
}

// 开始摄像头
const startCamera = async () => {
    try {
        const stream = await navigator.mediaDevices.getUserMedia({
            video: { width: 640, height: 480 }
        })

        const video = videoRef.value
        video.srcObject = stream

        await new Promise((resolve) => {
            video.onloadedmetadata = () => {
                video.play()
                resolve()
            }
        })

        cameraStarted.value = true

        // 设置Canvas尺寸
        await nextTick()
        const canvas = canvasRef.value
        canvas.width = video.videoWidth
        canvas.height = video.videoHeight
    } catch (error) {
        console.error('启动摄像头失败:', error)
    }
}

// 拍摄照片
const captureImage = () => {
    const video = videoRef.value
    const canvas = canvasRef.value
    const ctx = canvas.getContext('2d')

    ctx.drawImage(video, 0, 0, canvas.width, canvas.height)
    capturedImage.value = canvas.toDataURL('image/jpeg')
    detectedFace.value = null
    recognitionResult.value = null
}
const detectFaces = async () => {
    if (!capturedImage.value) return

    try {
        const base64Data = capturedImage.value.split(',')[1]
        const binaryData = atob(base64Data)
        const bytes = new Uint8Array(binaryData.length)

        for (let i = 0; i < binaryData.length; i++) {
            bytes[i] = binaryData.charCodeAt(i)
        }

        // 调用后端检测人脸
        const faces = await invoke('detect_faces', { imageData: bytes })
        console.log('检测到的人脸:', faces)

        if (faces && faces.length > 0) {
            const canvas = canvasRef.value
            const ctx = canvas.getContext('2d')
            ctx.clearRect(0, 0, canvas.width, canvas.height)

            const img = new Image()
            img.src = capturedImage.value

            img.onload = () => {
                ctx.drawImage(img, 0, 0, canvas.width, canvas.height)
                ctx.strokeStyle = 'green'
                ctx.lineWidth = 2

                for (const face of faces) {
                    // 绘制人脸框
                    ctx.beginPath()
                    ctx.rect(face.x, face.y, face.width, face.height)
                    ctx.stroke()

                    // 添加人脸标签
                    ctx.fillStyle = 'green'
                    ctx.font = '16px Arial'
                    ctx.fillText('人脸', face.x, face.y - 5)

                    // 绘制关键点
                    ctx.fillStyle = 'red'
                    let landmarkIndex = 1
                    for (const point of face.landmarks) {
                        ctx.beginPath()
                        ctx.arc(point.x, point.y, 3, 0, 2 * Math.PI)
                        ctx.fill()

                        // 添加关键点序号
                        ctx.fillStyle = 'blue'
                        ctx.font = '12px Arial'
                        ctx.fillText(`${landmarkIndex}`, point.x + 5, point.y - 5)
                        ctx.fillStyle = 'red'

                        landmarkIndex++
                    }
                }
            }

            detectedFace.value = faces[0]
        } else {
            alert('未检测到人脸')
        }
    } catch (error) {
        console.error('人脸检测失败:', error)
        alert(`人脸检测失败: ${error.message}`)
    }
}
const recognizeFace = async () => {
    if (!detectedFace.value || !capturedImage.value) return

    try {
        // 1. 获取拍摄图像的实际宽高（关键：用于校验人脸区域是否超出范围）
        const img = new Image();
        img.src = capturedImage.value;
        await new Promise(resolve => { img.onload = resolve; }); // 等待图像加载完成
        const imageWidth = img.width;
        const imageHeight = img.height;

        // 2. 修正人脸区域坐标和尺寸（与saveFace保持一致的逻辑）
        let { x, y, width, height } = detectedFace.value;
        // 转换为整数（避免浮点数误差）
        x = Math.round(x);
        y = Math.round(y);
        width = Math.round(width);
        height = Math.round(height);

        // 确保坐标非负
        x = Math.max(0, x);
        y = Math.max(0, y);

        // 确保宽高不超出图像范围
        width = Math.min(width, imageWidth - x);
        height = Math.min(height, imageHeight - y);

        // 确保宽高至少为1（避免无效区域）
        width = Math.max(1, width);
        height = Math.max(1, height);

        // 3. 处理图像数据（与之前一致）
        const base64Data = capturedImage.value.split(',')[1];
        const binaryData = atob(base64Data);
        const bytes = new Uint8Array(binaryData.length);
        for (let i = 0; i < binaryData.length; i++) {
            bytes[i] = binaryData.charCodeAt(i);
        }

        // 4. 使用修正后的人脸区域提取特征（关键修改：传入修正后的参数）
        const currentFeatures = await invoke('extract_features', {
            imageData: bytes,
            face: { x, y, width, height, landmarks: detectedFace.value.landmarks } // 修正后的区域
        });

        if (!currentFeatures?.features) {
            alert('无法提取人脸特征');
            return;
        }

        // 5. 与人脸库特征对比（与之前一致）
        let bestMatch = null;
        let highestSimilarity = 0;

        for (const face of faceDatabase.value) {
            const similarity = await invoke('compare_features', {
                features1: currentFeatures.features,
                features2: face.features
            });

            if (similarity > highestSimilarity) {
                highestSimilarity = similarity;
                bestMatch = face;
            }
        }

        recognitionResult.value = bestMatch && highestSimilarity > 0.5
            ? { name: bestMatch.name, similarity: highestSimilarity }
            : { name: '未知人脸', similarity: highestSimilarity || 0 };
    } catch (error) {
        console.error('人脸识别失败:', error);
        alert(`人脸识别失败: ${error.message}`);
    }
};

const saveFace = async () => {
    if (!detectedFace.value || !capturedImage.value || !faceName.value.trim()) {
        alert('请输入人脸名称');
        return;
    }

    try {
        // 1. 解析原始图像尺寸（关键：获取拍摄照片的实际宽高）
        const img = new Image();
        img.src = capturedImage.value;
        await new Promise(resolve => { img.onload = resolve; });
        const imageWidth = img.width;
        const imageHeight = img.height;

        // 2. 获取人脸区域参数并修正（核心：处理边界问题）
        let { x, y, width, height } = detectedFace.value;
        // 转换为整数（避免浮点数导致计算误差）
        x = Math.round(x);
        y = Math.round(y);
        width = Math.round(width);
        height = Math.round(height);

        // 确保x、y不小于0
        x = Math.max(0, x);
        y = Math.max(0, y);

        // 确保宽度不超过图像右边界
        width = Math.min(width, imageWidth - x);
        // 确保高度不超过图像下边界
        height = Math.min(height, imageHeight - y);

        // 确保宽高至少为1（避免无效区域）
        width = Math.max(1, width);
        height = Math.max(1, height);

        // 3. 提取图像数据（和之前一致）
        const base64Data = capturedImage.value.split(',')[1];
        const binaryData = atob(base64Data);
        const bytes = new Uint8Array(binaryData.length);
        for (let i = 0; i < binaryData.length; i++) {
            bytes[i] = binaryData.charCodeAt(i);
        }

        // 4. 提取人脸特征（使用修正后的人脸区域）
        const features = await invoke('extract_features', {
            imageData: bytes,
            face: { x, y, width, height, landmarks: detectedFace.value.landmarks } // 使用修正后的参数
        });

        if (!features?.features) {
            alert('无法提取人脸特征');
            return;
        }

        // 5. 绘制人脸缩略图（使用修正后的区域）
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        canvas.width = width;
        canvas.height = height;

        img.onload = () => { // 重新使用修正后的坐标绘制
            ctx.drawImage(img, x, y, width, height, 0, 0, width, height);
        };
        const thumbnailDataUrl = canvas.toDataURL('image/jpeg');

        // 6. 保存到人脸库
        faceDatabase.value.push({
            name: faceName.value.trim(),
            image: thumbnailDataUrl,
            features: features.features,
            timestamp: new Date().toISOString()
        });

        await saveFaceDatabase();
        alert('人脸保存成功');
        faceName.value = '';
    } catch (error) {
        console.error('保存人脸失败:', error);
        alert(`保存人脸失败: ${error.message}`);
    }
};

// 删除人脸
const deleteFace = async (index) => {
    if (confirm(`确定要删除 "${faceDatabase.value[index].name}" 的人脸数据吗？`)) {
        faceDatabase.value.splice(index, 1)
        await saveFaceDatabase()
    }
}

// 清除结果
const clearResults = () => {
    const canvas = canvasRef.value
    const ctx = canvas.getContext('2d')
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    capturedImage.value = null
    detectedFace.value = null
    recognitionResult.value = null
}

// 初始化
onMounted(async () => {
    await initializeFaceDatabase()
})

// 组件卸载时停止摄像头
onUnmounted(() => {
    if (cameraStarted.value && videoRef.value) {
        const stream = videoRef.value.srcObject
        stream.getTracks().forEach(track => track.stop())
        videoRef.value.srcObject = null
    }
})
</script>
<style scoped>
.face-detection-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px;
    max-width: 1200px;
    margin: 0 auto;
}

.camera-container {
    position: relative;
    width: 640px;
    height: 480px;
    margin-bottom: 20px;
    border: 1px solid #ddd;
    border-radius: 8px;
    overflow: hidden;
}

.camera-container video,
.camera-container canvas {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
}

.controls {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-bottom: 20px;
}

.controls button {
    padding: 10px 15px;
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s;
    display: flex;
    align-items: center;
}

.controls button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
}

.controls button:hover:not(:disabled) {
    background-color: #45a049;
}

.face-name-input {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 20px;
}

.face-name-input input {
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
    width: 200px;
}

.results,
.face-database {
    width: 100%;
    margin-bottom: 20px;
    background-color: #f9f9f9;
    padding: 15px;
    border-radius: 8px;
}

.result-item {
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.result-item.empty {
    color: #999;
    text-align: center;
}

.face-items {
    display: flex;
    flex-wrap: wrap;
    gap: 15px;
}

.face-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 120px;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    background-color: white;
}

.face-item img {
    width: 100px;
    height: 100px;
    object-fit: cover;
    margin-bottom: 10px;
    border-radius: 4px;
}

.face-item p {
    margin: 0;
    margin-bottom: 5px;
    font-weight: bold;
}

.face-item button {
    padding: 5px 10px;
    background-color: #f44336;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s;
}

.face-item button:hover {
    background-color: #d32f2f;
}
</style>