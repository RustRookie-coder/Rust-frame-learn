<script setup lang="ts">

import {Plus} from "@element-plus/icons-vue";
import {ref} from "vue";
import ImageMain from "@/components/Image/ImageMain.vue";
import ImageAside from "@/components/Image/ImageAside.vue";

const dialogVisible = ref(false)

const open = () => {
  dialogVisible.value = true
}
const close = () => {
  dialogVisible.value = false
}

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

const props = defineProps({
  modelValue: [String, Array]
})

let urls = []
const handleChoose = (e) => {
  urls = e.map(o => o.url)
  console.log(urls)
}

const emit = defineEmits(["update:modelValue"])
const submit = () => {
  if(urls.length) {
    emit("update:modelValue", urls[0])
  }
  close()
}

</script>

<template>
  <div v-if="modelValue">
    <el-image :src="modelValue" fit="cover" class="w-[100px] h-[100px] rounded border mr-3"></el-image>
  </div>
  <div v-else class="choose-image-btn" @click="open">
    <el-icon :size="25" class="text-gray-500"><Plus/></el-icon>
  </div>
  <el-dialog title="选择图片"
             v-model="dialogVisible"
             width="80%"
             top="5vh">
    <el-container class="bg-white rounded" style="height:70vh">
      <el-header class="image-header">
        <el-button type="primary" size="default" @click="handleOpenCreate">新增图片分类</el-button>
        <el-button type="warning" size="default" @click="handleOpenUpload">上传图片</el-button>
      </el-header>
      <el-container>
        <ImageAside ref="ImageSideRef" @change="handleAsideChange" />
        <ImageMain openChoose ref="ImageMainRef" @choose="handleChoose"/>
      </el-container>
    </el-container>
    <template #footer>
      <span>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="submit">确认</el-button>
      </span>
    </template>
  </el-dialog>
</template>

<style>
.image-header {
  border-bottom: 1px solid #eeeeee;
  @apply flex items-center;
}
.choose-image-btn {
  @apply w-[100px] h-[100px] rounded border flex justify-center items-center
  cursor-pointer hover:(br-gray-100);
}
</style>