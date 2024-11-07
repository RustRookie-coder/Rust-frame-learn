<script setup lang="ts">

import {Close, Edit} from "@element-plus/icons-vue";

const props = defineProps({
  active: {
    type: Boolean,
    default: false
  }
})

defineEmits(["edit", "delete"])
</script>

<template>
  <div class="aside-list" :class="{ 'active': props.active }">
    <span class="truncate"><slot/></span>
    <el-button class="ml-auto px-1" text type="primary" size="large" @click.stop="$emit('edit')">
      <el-icon :size="12">
        <Edit/>
      </el-icon>
    </el-button>
<!--    成功阻止冒泡事件-->
    <span @click.stop="() => {}">
    <el-popconfirm title="是否要删除该分类?"
                   confirm-button-text="确认"
                   cancel-button-text="取消"
                   @confirm.stop="$emit('delete')">
      <template #reference>
        <el-button text class="px-1" type="primary" size="default">
          <el-icon :size="12">
            <Close/>
          </el-icon>
        </el-button>
      </template>
    </el-popconfirm>
    </span>
  </div>
</template>

<style>
.aside-list {
  border-bottom: 1px solid #f4f4f4;
  cursor: pointer;
  @apply flex items-center p-3 text-sm text-gray-600;
}

.aside-list:hover {
  @apply bg-blue-50;
}

.aside-list:hover, .active {
  @apply bg-blue-50;
}
</style>