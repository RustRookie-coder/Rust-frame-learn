<script setup lang="ts">
</script>

<template>
  <el-config-provider>
    <router-view/>
  </el-config-provider>
</template>
<style>
body {
  @apply bg-gray-100;
}
#nprogress .bar {
  background-color: #f4f4f4 !important;
  height: 3px!important;
}
</style>
