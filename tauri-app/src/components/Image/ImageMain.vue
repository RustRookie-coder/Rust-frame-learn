<script setup lang="ts">
import {computed, ref} from "vue";
import {showPrompt} from "@/utils/common";
import UploadFile from "@/components/UploadFile.vue";
import { ElMessage } from "element-plus";

// page
const currentPage = ref(2)
const total = ref(0)
const limit = ref(10)

const imageList = ref([])
const image_class_id = ref(0)
const loading = ref(false)
const drawer = ref(false)

const mock = [{
  id: 1,
  url: "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c5166b58f.png",
  name: "1.png",
  checked: false,
}, {
  id: 2,
  url: "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c4e93b445.jpg",
  name: "2.jpg",
  checked: false,
}, {
  id: 3,
  url: "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c4e90c768.jpg",
  name: "3.jpg",
  checked: true,
}, {
  id: 4,
  url: "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c4e8d0b86.jpg",
  name: "4.jpg",
  checked: true,
}, {
  id: 5,
  url: "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c4e8a6a04.jpg",
  name: "5.jpg",
  checked: true,
}, {
  id: 6,
  url: "",
  name: "6.jpg",
  checked: false,
}, {
  id: 7,
  url: "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6273658e4a974.jpg",
  name: "7.jpg",
  checked: false,
}]

const getData = (page = null) => {
  if (typeof page == "number") {
    currentPage.value = page
  }

  loading.value = true
  imageList.value = mock
  //todo get image list func to backend
  loading.value = false
}

//根据分类id 重新加载图片列表
const loadData = (id) => {
  currentPage.value = 1
  image_class_id.value = id
  getData()
}

const handleEdit = (item) => {
  showPrompt("重命名", item.name).then((val) => {
    //todo 更新信息到后端
  })
}

const handleDelete = (item) => {
  console.log("删除成功" + item.id)
}

const openUploadFile = () => {
  drawer.value = true
}

const handleUploadSuccess = () => {
  getData(1)
}
const emit = defineEmits(["choose"])
const checkedImage = computed(() => imageList.value.filter(o => o.checked))
const handleChooseChange = (item) => {
  if(item.checked && checkedImage.value.length > 1) {
    item.checked = false
    ElMessage({
      message: '最多只能选择一张图片',
      type: 'error',
      duration: 3000
    })
    return
  }
  emit("choose", checkedImage.value)
}

const props = defineProps({
  openChoose: {
    type:Boolean,
    default: false
  }
})

defineExpose({
  loadData,
  openUploadFile
})

</script>
<template>
  <el-main class="image-main">
    <div class="top p-3">
      <el-row :gutter="10">
        <el-col :span="6" :offset="0" v-for="(item, index) in imageList" :key="index">
          <el-card shadow="hover" class="relative mb-3" :body-style="{'padding': 0}" :class="{ 'border-blue-500':item.checked }">
            <el-image :src="item.url" fit="cover" class="h-[245px]"
                      style="width: 100%"
                      :preview-src-list="[item.url]"
                      :initial-index="0"
            ></el-image>
            <div class="image-title">{{ item.name }}</div>
            <div class="flex items-center justify-center p-2">

              <el-checkbox v-if="props.openChoose" v-model="item.checked" @change="handleChooseChange(item)"/>
              <el-button type="primary" size="default" text @click="handleEdit(item)">重命名</el-button>
              <el-popconfirm title="是否要删除该图片" confirm-button-text="确认" cancel-button-text="取消" @confirm="handleDelete(item)">
                <template #reference>
                  <el-button type="primary" size="default" text>删除</el-button>
                </template>
              </el-popconfirm>
            </div>
          </el-card>
        </el-col>
      </el-row>
    </div>
    <div class="bottom">
      <el-pagination background layout="prev, pager, next"
                     :total="total"
                     :current-page="currentPage"
                     :page-size="limit"
                     @current-change="getData"
      />
    </div>
  </el-main>
  <el-drawer v-model="drawer"
             title="上传图片">
  <UploadFile :data="image_class_id" @success="handleUploadSuccess"/>
  </el-drawer>
</template>
<style>
.image-main {
  position: relative;
}

.image-main .top {
  position: absolute;
  top: 0;
  right: 0;
  left: 0;
  bottom: 50px;
  overflow-y: auto;
}

.image-main .bottom {
  position: absolute;
  bottom: 0;
  height: 50px;
  left: 0;
  right: 0;
  @apply flex items-center justify-center;
}

.image-title {
  position: absolute;
  top: 218px;
  left: -1px;
  right: -1px;
  @apply text-sm truncate text-gray-100 bg-opacity-50 bg-gray-800 px-2 py-1;
}
</style>