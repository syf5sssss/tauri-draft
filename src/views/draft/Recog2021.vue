<template>
    <div class="face-recognition">
        <div class="controls">
            <input v-model="newFaceName" placeholder="输入姓名" />
            <input type="file" accept="image/*" @change="handleImageSelect" ref="fileInput" />
            <button @click="addFace" :disabled="!newFaceName || !selectedImage">录入人脸</button>
            <button @click="startRecognition" :disabled="recognitionActive">开始识别</button>
            <button @click="stopRecognition" :disabled="!recognitionActive">停止识别</button>
            <!-- 添加拍摄按钮 -->
            <button @click="captureFrame" :disabled="!recognitionActive">拍摄</button>
        </div>

        <div class="camera-view">
            <img :src="videoFrame" alt="Camera Feed" v-if="videoFrame" />
            <div v-else class="placeholder">摄像头未启动</div>
        </div>

        <div class="face-gallery">
            <div v-for="face in faces" :key="face.name" class="face-card">
                <img :src="faceImageUrls[face.name]" alt="Face" v-if="faceImageUrls[face.name]" />
                <div>{{ face.name }}</div>
                <button @click="deleteFace(face.name)">删除</button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// 状态管理
const newFaceName = ref('');
const selectedImage = ref(null);
const videoFrame = ref('');
const faces = ref([]);
const recognitionActive = ref(false);
const faceImageUrls = ref({}); // 缓存图片URL
const fileInput = ref(null); // 文件输入引用

// 生成人脸图片的URL（通过后端接口获取）
const loadFaceImage = async (face) => {
    try {
        // 调用后端接口获取图片的Base64编码
        const base64Str = await invoke('get_face_image', { path: face.image_path });
        // 生成图片URL
        const base64Url = `data:image/jpeg;base64,${base64Str}`;
        // 缓存结果
        faceImageUrls.value[face.name] = base64Url;
        return base64Url;
    } catch (error) {
        console.error('加载图片失败:', error);
        return 'data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEASABIAAD...'; // 默认占位图
    }
};

// 获取所有人脸数据
const loadFaces = async () => {
    try {
        const result = await invoke('get_all_faces');
        faces.value = result;

        // 预加载所有图片
        await Promise.all(result.map(face => loadFaceImage(face)));
    } catch (error) {
        console.error('加载人脸数据失败:', error);
    }
};

// 处理图片选择
const handleImageSelect = (e) => {
    const file = e.target.files[0];
    if (file) {
        selectedImage.value = file;
    }
};

// 添加人脸
const addFace = async () => {
    if (!newFaceName.value.trim() || !selectedImage.value) return;

    try {
        // 读取图片文件为二进制数据
        const arrayBuffer = await selectedImage.value.arrayBuffer();
        const imageData = new Uint8Array(arrayBuffer);

        // 调用后端接口
        const result = await invoke('add_face', {
            name: newFaceName.value,
            imageData: Array.from(imageData),
            fileName: selectedImage.value.name
        });

        console.log(result);
        // 重置表单
        newFaceName.value = '';
        selectedImage.value = null;
        // 清空文件输入
        if (fileInput.value) {
            fileInput.value.value = '';
        }
        // 重新加载人脸列表
        await loadFaces();
    } catch (error) {
        console.error('添加人脸失败:', error);
        alert(`添加失败: ${error.message || error}`);
    }
};

// 删除人脸
const deleteFace = async (name) => {
    if (!confirm(`确定要删除 ${name} 吗？`)) return;

    try {
        const result = await invoke('delete_face', { name });
        console.log(result);
        // 从缓存中移除
        delete faceImageUrls.value[name];
        await loadFaces();
    } catch (error) {
        console.error('删除人脸失败:', error);
        alert(`删除失败: ${error.message || error}`);
    }
};

// 开始实时识别
const startRecognition = async () => {
    if (recognitionActive.value) return;

    try {
        recognitionActive.value = true;
        const result = await invoke('start_recognition');
        console.log(result);
    } catch (error) {
        console.error('开始识别失败:', error);
        recognitionActive.value = false;
        alert(`启动失败: ${error.message || error}`);
    }
};

// 停止识别
const stopRecognition = () => {
    recognitionActive.value = false;
    videoFrame.value = '';
};

// 拍摄当前画面
const captureFrame = async () => {
    if (!videoFrame.value) return;

    try {
        // 从base64字符串中提取图片数据
        const base64Data = videoFrame.value.split(',')[1];

        // 调用后端保存图片
        const fileName = `capture_${Date.now()}.jpg`;
        await invoke('save_captured_frame', {
            base64Data,
            fileName
        });

        // 创建File对象用于录入人脸
        const byteCharacters = atob(base64Data);
        const byteNumbers = new Array(byteCharacters.length);
        for (let i = 0; i < byteCharacters.length; i++) {
            byteNumbers[i] = byteCharacters.charCodeAt(i);
        }
        const byteArray = new Uint8Array(byteNumbers);
        const blob = new Blob([byteArray], { type: 'image/jpeg' });

        // 创建File对象
        selectedImage.value = new File([blob], fileName, {
            type: 'image/jpeg',
            lastModified: Date.now()
        });

        alert('拍摄成功！请填写姓名后点击"录入人脸"');
    } catch (error) {
        console.error('拍摄失败:', error);
        alert(`拍摄失败: ${error.message || error}`);
    }
};

// 监听视频帧事件
let unlistenVideoFrame;
onMounted(async () => {
    await loadFaces();

    // 监听后端发送的视频帧
    unlistenVideoFrame = await listen('video-frame', (event) => {
        if (recognitionActive.value) {
            videoFrame.value = `data:image/jpeg;base64,${event.payload}`;
        }
    });
});

onBeforeUnmount(() => {
    if (unlistenVideoFrame) {
        unlistenVideoFrame();
    }
    stopRecognition();
});
</script>

<style scoped>
/* 样式保持不变 */
.face-recognition {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 20px;
    box-sizing: border-box;
    gap: 20px;
}

.controls {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
    align-items: center;
}

.controls input[type="text"] {
    flex: 1;
    min-width: 200px;
    padding: 8px;
}

.controls button {
    padding: 8px 16px;
    cursor: pointer;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
}

.controls button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
}

.camera-view {
    flex: 1;
    border: 1px solid #ccc;
    min-height: 480px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f5f5f5;
    position: relative;
}

.camera-view img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
}

.placeholder {
    color: #999;
    font-style: italic;
}

.face-gallery {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 15px;
    margin-top: 20px;
    max-height: 200px;
    overflow-y: auto;
    padding: 10px;
    border: 1px solid #eee;
}

.face-card {
    border: 1px solid #eee;
    border-radius: 8px;
    padding: 10px;
    text-align: center;
    background: white;
}

.face-card img {
    width: 100px;
    height: 100px;
    object-fit: cover;
    border-radius: 4px;
    margin-bottom: 8px;
}

.face-card button {
    margin-top: 8px;
    padding: 4px 8px;
    background-color: #ff4444;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
}

.face-card button:hover {
    background-color: #cc0000;
}
</style>