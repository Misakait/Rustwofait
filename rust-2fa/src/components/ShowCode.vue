<template>
  <div class="card-container h-full">

    <!-- 遍历数据列表，渲染卡片 -->
    <div v-for="(item, index) in store.dataList" :key="index" class="card flex flex-row">
      <div class="basis-1/4">
        <div class="Capital">
          {{ item.company[0] }}
        </div>
      </div>
      <div class="basis-3/4">
        <div>{{ item.company }}&nbsp;&nbsp;{{ item.username }}&nbsp;&nbsp;{{ item.custom_name }}</div>
        <div class="text-2xl font-semibold code">{{ formattedTotpCode(item.totp_code)}}</div>
        <div></div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { onMounted} from 'vue';
import {invoke} from "@tauri-apps/api/core";
import {useTotoStore} from "../stores/Toto.ts";
import { TwoFactorAuthInfoType } from "../impl/TwoFactorAuthInfoType.ts";
const store = useTotoStore();
// 定义数据列表，每个数据项包含两个文本信息


// 计算属性函数，用于格式化 totp_code
const formattedTotpCode = (code: number) => {
  const codeStr = code.toString();
  if (codeStr.length === 6) {
    return codeStr.slice(0, 3) + " " + codeStr.slice(3);
  } else if (codeStr.length === 8) {
    return codeStr.slice(0, 4) + " " + codeStr.slice(4);
  }
  return codeStr;
};
onMounted(async () => {
  store.dataList= await invoke<Array<TwoFactorAuthInfoType>>('generate_totp');
  store.remainingTime = store.dataList[0].remaining_time;
})

</script>

<style scoped>

.card-container {
  /* 使用 flex 布局 */
  display: flex;
  /* 允许换行 */
  flex-wrap: wrap;
  /* 卡片之间的间距 */
  /*gap: 10px;*/
  background-image: linear-gradient(to top, #fff1eb 0%, #ace0f9 100%);
}

.card {
  margin: 5px;
  height: 80px;
  /* 每个卡片的宽度，每行放两个卡片 */
  width: calc(50% - 10px);
  /* 卡片的边框 */
 /* border: 1px solid #ccc;*/
  /* 卡片的内边距 */
  padding: 10px;
  /* 卡片的盒模型计算方式 */
  box-sizing: border-box;
  background-color: rgba(255, 255, 255, 0.25);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
  border: 0.666667px solid rgba(255, 255, 255, 0.18);
  box-shadow: rgba(142, 142, 142, 0.19) 0px 6px 15px 0px;
  -webkit-box-shadow: rgba(142, 142, 142, 0.19) 0px 6px 15px 0px;
  border-radius: 12px;
  -webkit-border-radius: 12px;
}

.card input {
  /* 文本框的宽度 */
  width: 100%;
  /* 文本框的外边距 */
  margin-bottom: 5px;
}
.Capital{
  width: 60px;
  height: 60px;
  border-radius: 50%;
  background-color: #9c88ff;
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  font-size: 24px;
  margin-right: 5px;
}
.code{
  color: #0452D6;
}
</style>