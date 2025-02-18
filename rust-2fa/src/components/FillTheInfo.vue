<template>
  <el-card style="max-width: 480px" id="container">
    <template #header>
      <div class="card-header">
        <el-input v-model="input" style="width: 240px" placeholder="输入自定义名称(可选)" />
      </div>
    </template>
    <div>
      <span>公司名:&nbsp;&nbsp;</span>
      <span>{{store.company}}</span>
    </div>
    <div>
      <span>
        <span>账户名:&nbsp;&nbsp;</span>
        <span>{{store.accountName}}</span>
      </span>
    </div>
    <template #footer>
      <el-button type="primary" :disabled="disabled" @click="save">保存</el-button>
    </template>
  </el-card>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue';
import {useTotoStore} from "../stores/Toto.ts";
import {invoke} from "@tauri-apps/api/core";
import {TwoFactorAuthInfoType} from "../impl/TwoFactorAuthInfoType.ts";
const input = ref('')
const store = useTotoStore()


const disabled = computed(() => {
  return !store.company && !store.accountName;
});

const save = async () => {
  try{
  const result = await invoke<string>('save_2fa_data', {
    name: input.value,
    otpauth: store.otpauth
  });
  store.dataList = await invoke<Array<TwoFactorAuthInfoType>>('generate_totp');
  store.remainingTime = store.dataList[0].remaining_time;
   alert(result)
    // 清空数据
    store.company = '';
    store.accountName = '';
    input.value = '';
    store.previewUrl = '';
    store.otpauth = '';
} catch (error) {
  console.error('调用 Rust 命令出错:', error);
}

};
</script>

<style scoped>
#container {
  max-height: 100%;
  height: 40vh;
}
</style>