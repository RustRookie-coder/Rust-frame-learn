<template>
  <el-container class="bg-white rounded" :style="{ height:(h + 'px') }">
    <el-header class="image-header">
      <el-button type="primary" size="default" @click="handleOpenCreate">新增图片分类</el-button>
      <el-button type="warning" size="default" @click="handleOpenUpload">上传图片</el-button>
    </el-header>
    <el-container>
      <ImageAside ref="ImageSideRef" @change="handleAsideChange" />
      <ImageMain ref="ImageMainRef"/>
    </el-container>
  </el-container>

</template>
<script setup lang="ts">
import ImageAside from "@/components/Image/ImageAside.vue";
import ImageMain from "@/components/Image/ImageMain.vue";
import {ref} from "vue";

const windowHeight = window.innerHeight || document.body.clientHeight
const h = windowHeight - 64 - 44 - 40

const ImageSideRef = ref(null)
const ImageMainRef = ref(null)

const handleOpenCreate = () => ImageSideRef.value.handleCreate()

const handleAsideChange = (image_class_id) => {
  ImageMainRef.value.loadData(image_class_id)
  console.log("分类id" + image_class_id)
}

const handleOpenUpload = () => {
  ImageMainRef.value.openUploadFile()
}

</script>
<style>
.image-header {
  border-bottom: 1px solid #eeeeee;
  @apply flex items-center;
}
</style>