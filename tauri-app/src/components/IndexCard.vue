<template>
  <el-card shadow="never">
    <template #header>
      <div class="flex justify-between">
        <span class="text-sm">{{ props.title }}</span>
        <el-tag type="danger" effect="plain">
          {{ props.tip }}
        </el-tag>
      </div>
    </template>
    <el-row :gutter="20">
      <el-col :span="6" :offset="0" v-for="(item, index) in props.columnData" :key="index">
        <el-card shadow="hover" class="border-0 bg-light-400">
          <div class="flex flex-col items-center justify-between">
            <span class="text-x1 mb-2">{{ item.value }}</span>
            <span class="text-sm text-gray-500">{{ item.label }}</span>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </el-card>
</template>
<script setup lang="ts">

const props = defineProps({
  title: String,
  tip: String,
  columnData: Array,
})
</script>
<style scoped>

</style>