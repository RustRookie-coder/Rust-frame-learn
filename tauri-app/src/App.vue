<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import TodoDemo from "@/components/TodoDemo.vue";
</script>

<template>
  <el-config-provider>
    <router-view/>
  </el-config-provider>
</template>
<style>
#nprogress .bar {
  background-color: #f4f4f4 !important;
}
</style>
