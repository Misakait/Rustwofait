<script setup lang="ts">

import DragAndDropUpload from "./DragAndDropUpload.vue";
</script>

<template>
  <!-- 父容器 -->
  <div class="grid-container">
    <!-- 左边区域 -->
    <div class="left-section">
      <DragAndDropUpload />
    </div>
    <!-- 右边区域 -->
    <div >
      <FillTheInfo />
    </div>
  </div>
</template>

<style scoped>
/* 父容器样式 */
.grid-container {
  display: grid;
  /* 定义列布局，左边占 1 份，右边占 1 份 */
  grid-template-columns: 1fr 1fr;
}


</style>