<script setup lang="ts">

import {Refresh} from "@element-plus/icons-vue";

defineEmits(["create", "refresh"])
</script>

<template>
  <div class="flex items-center justify-between mb-4">
    <el-button type="primary" size="default" @click="$emit('create')">新增</el-button>
    <el-tooltip class="box-item" effect="dark" content="" placement="top">
      <el-button text @click="$emit('refresh')">
        <el-icon :size="20">
          <refresh/>
        </el-icon>
      </el-button>
    </el-tooltip>
  </div>
</template>

<style>

</style>