<script setup lang="ts">
  import {ref} from "vue";
  import * as iconList from '@element-plus/icons-vue'

  const props = defineProps({
    modelValue: String,
  })
  const icons = ref(Object.keys(iconList))

  const emit = defineEmits(['update:modelValue'])
  const handleChange = (icon) => {
    emit("update:modelValue", icon)
  }
</script>

<template>
  <div class="flex items-center">
    <el-icon :size="20" v-if="modelValue">
      <component :is="modelValue"/>
    </el-icon>
    <el-select :model-value="props.modelValue"
               placeholder="请选择图标"
               @change="handleChange"
               filterable
               clearable>
      <el-option
          v-for="item in icons"
          :key="item"
          :label="item"
          :value="item"
      >
        <div class="flex items-center justify-between">
          <el-icon>
            <component :is="item"/>
          </el-icon>
          <span class="text-gray-500">{{ item }}</span>
        </div>
      </el-option>
    </el-select>
  </div>
</template>

<style>

</style>