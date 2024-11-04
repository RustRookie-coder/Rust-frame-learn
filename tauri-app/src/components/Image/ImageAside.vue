<script setup lang="ts">

import AsideList from "@/components/AsideList.vue";
import {ref} from "vue";

const loading = ref(false)
const listData = [
  {
    "id": 133,
    "name": "电脑",
    "images_count": 10,
    "order": 1001,
  },
]
const list = ref([])
</script>

<template>
  <el-aside width="220px" class="image-aside" v-loading="loading">
    <div class="top">
      <AsideList v-for="(item, index) in list" :key="index">
        分类标题
      </AsideList>
    </div>
    <div class="bottom">
      <div>分页部分</div>
    </div>
  </el-aside>
</template>

<style>
.image-aside {
  border-right: 1px solid #eeeeee;
  position: relative;
}

.image-aside .top {
  position: absolute;
  top: 0;
  right: 0;
  left: 0;
  bottom: 50px;
  overflow-y: auto;
}

.image-aside .bottom {
  position: absolute;
  bottom: 0;
  height: 50px;
  left: 0;
  right: 0;
  @apply flex items-center justify-center;
}
</style>