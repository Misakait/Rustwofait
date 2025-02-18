<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";
import add2fa from "./components/add2fa.vue";
import ShowCode from "./components/ShowCode.vue";
import {  watchEffect } from 'vue';
import {useTotoStore} from "./stores/Toto.ts";
import {invoke} from "@tauri-apps/api/core";
import { TwoFactorAuthInfoType } from "./impl/TwoFactorAuthInfoType.ts";
const store = useTotoStore()
watchEffect(async () => {
  const interval = setInterval(async () => {
    if (store.remainingTime > 0) {
      store.remainingTime--;
    } else {
      store.dataList = await invoke<Array<TwoFactorAuthInfoType>>('generate_totp');
      store.remainingTime = store.dataList[0].remaining_time;
    }
  }, 1000);
  // 清除定时器，防止内存泄漏
  return () => clearInterval(interval);
});

</script>

<template>
  <!-- 父容器 -->
  <main class="grid grid-rows-[2fr_3fr] h-screen">
      <!-- 上面部分 -->
      <div class="border-4 border-indigo-200 border-b-0">
        <add2fa/>
      </div>
      <!-- 下面部分 -->

      <div class="border-4 border-indigo-200 border-t-0" >
        <div class="border-2 border-indigo-500 h-0 transition-all duration-1000 ease-linear" :style="{ width: (store.remainingTime / 30 * 100) + '%' }"></div>
        <ShowCode/>
      </div>
  </main>
</template>

<style scoped>

</style>

<style>
@import "tailwindcss";
*{
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}


</style>