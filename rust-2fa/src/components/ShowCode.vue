<template>
  <div class="card-container h-full">

    <!-- 遍历数据列表，渲染卡片 -->
<!--    <div v-for="(item, index) in store.dataList" :key="item.id" class="card flex flex-row">-->
    <div v-for="item in store.dataList" :key="item.id" class="card flex flex-row">
      <div class="basis-1/5">
        <div class="Capital">
          {{ item.company[0] }}
        </div>
      </div>
      <div class="basis-3/5">
        <div>{{ item.company }}&nbsp;&nbsp;{{ item.username }}&nbsp;&nbsp;{{ item.custom_name }}{{ item.id }}</div>
        <div class="text-2xl font-semibold code">{{ formattedTotpCode(item.totp_code)}}</div>
        <div></div>
      </div>
      <div class="basis-1/5">
        <button
            @click="toggleDropdown(item.id)"
            class="h-[60px] w-[60px] text-2xl flex items-center justify-center font-bold"
            :ref="(el) => setButtonRef(el as HTMLElement | null, item.id)"
        >
          ⋮
        </button>
        <!-- 下拉框 -->
        <div
            v-if="dropdownVisible[item.id]"
            class="absolute top-8 right-2 bg-white border border-gray-300 rounded shadow-md z-10"
            :ref="(el) => setDropdownRef(el as HTMLElement | null, item.id)"
        >
          <ul>
            <li
                @click="handleDelete(item.id)"
                class="hover:bg-gray-100 cursor-pointer"
            >
              删除
            </li>
<!--            <li-->
<!--                @click="handleEdit(index)"-->
<!--                class="hover:bg-gray-100 cursor-pointer"-->
<!--            >-->
<!--              更改-->
<!--            </li>-->
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import {ref, onMounted,onUnmounted} from 'vue';
import {invoke} from "@tauri-apps/api/core";
import {useTotoStore} from "../stores/Toto.ts";
import { TwoFactorAuthInfoType } from "../impl/TwoFactorAuthInfoType.ts";
const store = useTotoStore();
// 下拉框的显示状态
const dropdownVisible = ref<boolean[]>([]);
const buttonRefs = ref<Array<HTMLElement | null>>([]);
const dropdownRefs = ref<Array<HTMLElement | null>>([]);
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
  dropdownVisible.value = new Array(store.dataList.length).fill(false);
  // 点击文档时，隐藏所有下拉框
  document.addEventListener('click', handleDocumentClick);
})
onUnmounted(() => {
  // 移除文档点击事件监听，防止内存泄漏
  document.removeEventListener('click', handleDocumentClick);
});

// 点击文档时关闭下拉框的处理函数
const handleDocumentClick = (event: MouseEvent) => {
  dropdownVisible.value.forEach((visible, index) => {
    if (visible) {
      const dropdown = dropdownRefs.value[index];
      const button = buttonRefs.value[index];
      if (!dropdown?.contains(event.target as Node) && !button?.contains(event.target as Node)) {
        dropdownVisible.value[index] = false;
      }
    }
  });
};
const setButtonRef = (el: HTMLElement | null, index: number) => {
  buttonRefs.value[index] = el;
};

const setDropdownRef = (el: HTMLElement | null, index: number) => {
  dropdownRefs.value[index] = el;
};
// 切换下拉框的显示状态
const toggleDropdown = (index: number) => {
  dropdownVisible.value[index] =!dropdownVisible.value[index];
};

// 删除处理函数
const handleDelete = async (index: number) => {
  // 这里添加删除逻辑，例如调用后端接口或更新 store 中的数据
  // store.dataList.splice(index, 1);
  let result = await invoke<string>('delete_2fa_data', {
    id: index,
  });
  alert(result)
  dropdownVisible.value[index] = false;
  store.dataList= await invoke<Array<TwoFactorAuthInfoType>>('generate_totp');
  store.remainingTime = store.dataList[0].remaining_time;
};

// 更改处理函数
// const handleEdit = (index: number) => {
//   // 这里添加更改逻辑，例如弹出编辑框
//   dropdownVisible.value[index] = false;
//   console.log(`编辑第 ${index} 条数据`);
// };
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
li{
  padding: 10px;
}
</style>