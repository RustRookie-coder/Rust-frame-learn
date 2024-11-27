<script setup lang="ts">
import {ref} from "vue";
const showDrawer = ref(false)

const props = defineProps({
  title: String,
  size: {
    type: String,
    default: "50%"
  },
  destroyOnClose: {
    type: Boolean,
    default: false
  },
  confirmText: {
    type: String,
    default: "确定"
  }
})

const loading = ref(false)
const showLoading = () => loading.value = true
const hideLoading = () => loading.value = false
const open = () => {
  showDrawer.value = true
}

const close = () => {
  showDrawer.value = false
}
const emit = defineEmits(['submit'])
const submit = () => emit("submit")
//向父组件暴露方法
defineExpose({
  open,
  close,
  showLoading,
  hideLoading,
})
</script>

<template>
  <el-drawer v-model="showDrawer"
             :title="title"
             :size="size"
             :close-on-click-modal="false"
  :destroy-on-close="props.destroyOnClose">
    <div class="formDrawer">
      <div class="body">
        <slot></slot>
      </div>
      <div class="actions">
        <el-button type="primary" @click="submit" :loading="loading">{{ confirmText }}</el-button>
        <el-button type="text" @click="close">取消</el-button>
      </div>
    </div>
  </el-drawer>
</template>

<style>
  .formDrawer {
    width: 100%;
    height: 100%;
    position: relative;
    @apply flex flex-col;
  }

  .formDrawer .body {
    flex: 1;
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 50px;
    overflow-y: auto;
  }
  .formDrawer .actions {
    height: 50px;
    @apply mt-auto flex items-center;
  }
</style>