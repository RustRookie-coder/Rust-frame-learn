<template>
  <el-container class="bg-white rounded" :style="{ height:(h + 'px') }">
    <el-header class="image-header">Header</el-header>
    <el-container>
      <ImageAside/>
      <ImageMain/>
    </el-container>
  </el-container>

</template>
<script setup lang="ts">
import ImageAside from "@/components/Image/ImageAside.vue";
import ImageMain from "@/components/Image/ImageMain.vue";

const windowHeight = window.innerHeight || document.body.clientHeight
const h = windowHeight - 64 - 44
</script>

<style>
.image-header {
  border-bottom: 1px solid #eeeeee;
  @apply flex items-center ;
}


</style>