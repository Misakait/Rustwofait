<template>
  <div
      class="drop-area"
      :class="{ 'drop-area--dragging': isDragging }"
      @dragover.prevent
      @dragenter="handleDragEnter"
      @dragleave="handleDragLeave"
      @drop.prevent="handleDrop"
  >
    <p class="tips" v-if="!isDragging">将图片拖拽到这里上传</p>
    <p class="tips" v-else>松开鼠标上传图片</p>
<!--    <img class="img" v-if="store.previewUrl" :src="store.previewUrl" alt="Preview" />-->
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import jsQR from "jsqr";
import {invoke} from "@tauri-apps/api/core";
import {useTotoStore} from "../stores/Toto.ts";
// 用于标记是否正在拖拽，明确指定类型为 boolean
const isDragging = ref<boolean>(false);
// 存储图片预览的 URL，明确指定类型为 string
// 处理拖拽进入事件
const store = useTotoStore()

const handleDragEnter = (): void => {
  isDragging.value = true;
};

// 处理拖拽离开事件
const handleDragLeave = (): void => {
  isDragging.value = false;
};

// 处理文件拖拽放下事件
const handleDrop = async (event: DragEvent): Promise<void> => {
  isDragging.value = false;
  const files: FileList | undefined = event.dataTransfer?.files;
  if (files && files.length > 0) {
    const file: File = files[0];
    if (file.type.indexOf('image') > -1) {
      const reader: FileReader = new FileReader();
      reader.onload = async (e: ProgressEvent<FileReader>): Promise<void> => {
        store.previewUrl = e.target?.result as string;
        let otpUrl = await decodeQR(store.previewUrl);
        if (otpUrl === null ){
          return;
        }
        alert("识别成功")
        store.otpauth = otpUrl;
        try {
          // 调用 Tauri 命令将图片数据传递给 Rust 处理
          const result = await invoke<[string, string]>('parse_2fa_data', {
            otpauth: otpUrl,
          });
          const [company,accountName]  = result;
          store.company = company;
          store.accountName = accountName;

        } catch (error) {
          console.error('调用 Rust 命令出错:', error);
        }
      };
      reader.readAsDataURL(file);
    }
  }
};
const decodeQR = async (dataURL:string):Promise<string | null>=>{
  const img = new Image();
  img.src = dataURL;

  // 等待图像加载完成
  await new Promise((resolve) => {
    img.onload = resolve;
  });

  // 创建 canvas 并绘制图像
  const canvas = document.createElement('canvas');
  canvas.width = img.width;
  canvas.height = img.height;
  const ctx = canvas.getContext('2d');
  if (ctx) {
    ctx.drawImage(img, 0, 0, img.width, img.height);

    // 获取 ImageData 对象
    const imageData = ctx.getImageData(0, 0, img.width, img.height);

    // 使用 jsqr 识别二维码
    const code = jsQR(imageData.data, imageData.width, imageData.height);
    if (code) {
      console.log('识别到的二维码内容:', code.data);
      return code.data;
    } else {
      console.log('未识别到二维码');
      return null;
    }
  }
  return null;
}


</script>

<style scoped>
.drop-area{
  position: relative;
  height: 100%;
  max-width: 100%;
  max-height: 100%;
  background-image: url("../assets/upload.png");
  background-size: contain;
  background-position: center;
  background-repeat: no-repeat;
}
img{
  /* 限制图片最大宽度为容器宽度 */
  max-width: 100%;
  /* 限制图片最大高度为容器高度 */
  max-height: 100%;
  object-fit: cover;
}
.drop-area--dragging::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(128, 128, 128, 0.5);
  backdrop-filter: blur(5px);
}
.tips {
  position: absolute;
  bottom: 5px;
  left: 50%;
  /* 将元素向左移动自身宽度的一半 */
  transform: translateX(-50%);
}
</style>