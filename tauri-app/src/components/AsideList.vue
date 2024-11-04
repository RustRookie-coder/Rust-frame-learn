<script setup lang="ts">

import {Close, Edit} from "@element-plus/icons-vue";

defineProps({
  active: {
    type: Boolean,
    default: false
  }
})

defineEmits(["edit", "delete"])
</script>

<template>
  <div class="aside-list" :class="{ 'active': active }">
    <span class="truncate"><slot/></span>
    <el-button class="ml-auto px-1" text type="primary" size="large" @click="$emit('edit')">
      <el-icon :size="12"><Edit/></el-icon>
    </el-button>
    <el-button class="px-1" text type="primary" size="large" @click="$emit('delete')">
      <el-icon :size="12"><Close/></el-icon>
    </el-button>
  </div>
</template>

<style>
.aside-list{
  border-bottom: 1px solid #f4f4f4;
  cursor: pointer;
  @apply flex items-center p-3 text-sm text-gray-600;
}
.aside-list:hover{
  @apply bg-blue-50;
}
.aside-list:hover, active {
  @apply bg-blue-50;
}
</style>