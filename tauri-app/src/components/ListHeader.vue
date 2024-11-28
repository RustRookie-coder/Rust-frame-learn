<script setup lang="ts">

import {Refresh} from "@element-plus/icons-vue";
import {computed} from "vue";

defineEmits(["create", "refresh", "delete"])

const props = defineProps({
  layout: {
    type: String,
    default: "create, refresh"
  }
})

const btns = computed(() => props.layout)
</script>

<template>
  <div class="flex items-center justify-between mb-4">
    <div>
      <el-button v-if="btns.includes('create')" type="primary" size="default" @click="$emit('create')">新增</el-button>
      <el-popconfirm title="是否要删除选中记录?"
                     confirm-button-text="确认"
                     cancel-button-text="取消"
                     v-if="btns.includes('delete')"
                     @confirm="$emit('delete')">
        <template #reference>
          <el-button type="danger" size="default">
            批量删除
          </el-button>
        </template>
      </el-popconfirm>
    </div>
    <el-tooltip class="box-item" effect="dark" content="" placement="top">
      <el-button v-if="btns.includes('refresh')" text @click="$emit('refresh')">
        <el-icon :size="20">
          <refresh/>
        </el-icon>
      </el-button>
    </el-tooltip>
  </div>
</template>

<style>

</style>