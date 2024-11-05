<script setup lang="ts">

import {UploadFilled} from "@element-plus/icons-vue";
import {ref} from "vue";

//todo need to get token
const token = ref("token")

const props = defineProps({
  data: Object
})

const emit = defineEmits(["success"])

const uploadSuccess = (response, uploadFile, uploadFiles) => {
  emit("success", {
    response, uploadFile, uploadFiles
  })
}

const uploadError = (error, uploadFile, uploadFiles) => {
  console.log(error)
}
</script>

<template>
  <el-upload class="upload-demo" drag action="https://jsonplaceholder.typicode.com/posts/"
             multiple
             :headers="{ token }"
             name="img"
             :data="props.data"
             :on-success="uploadSuccess"
             :on-error="uploadError"
  >
    <el-icon class="el-icon--upload">
      <upload-filled/>
    </el-icon>

    <div class="el-upload__text">
      Drop file
    </div>
    <template #tip>
      <div class="el-upload__tip">
        jpg/png files with a size less than 500kb
      </div>
    </template>
  </el-upload>
</template>

<style>

</style>