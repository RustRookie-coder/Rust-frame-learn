<script setup lang="ts">

import AsideList from "@/components/AsideList.vue";
import {computed, reactive, ref} from "vue";
import FormDrawer from "@/components/FormDrawer.vue";

const loading = ref(false)
const listData = [
  {
    "id": 1,
    "name": "运动旅行",
    "images_count": 10,
    "order": 1001,
  },
  {
    "id": 2,
    "name": "居家生活",
    "images_count": 10,
    "order": 1001,
  },
  {
    "id": 3,
    "name": "手机数码",
    "images_count": 10,
    "order": 1001,
  }, {
    "id": 4,
    "name": "智能家电",
    "images_count": 10,
    "order": 1001,
  }, {
    "id": 5,
    "name": "化妆品",
    "images_count": 10,
    "order": 1001,
  },
  {
    "id": 6,
    "name": "健身器材",
    "images_count": 10,
    "order": 1001,
  },
  {
    "id": 133,
    "name": "电脑",
    "images_count": 10,
    "order": 1001,
  },
]
const list = ref(listData)
const currentPage = ref(2)
const total = ref(0)
const limit = ref(10)
const activeId = ref(0)

const form = reactive({
  name: "",
  order: 50
})

const rules = {
  name: [{
    required: true,
    message: '图库分类不能为空',
    trigger: 'blur'
  }]
}

const formRef = ref(null)

let item = list.value[0]
if (item) {

}

const getData = (p) => {
  if (typeof p == "number") {
    currentPage.value = p
  }
  console.log(p)
}

const formDrawerRef = ref(null)
const editId = ref(0)
const drawerTitle = computed(() => {
  return editId.value > 0 ? "修改" : "新增"
})
const handleCreate = () => {
  editId.value = 0
  form.name = ""
  form.order = 50
  formDrawerRef.value.open()
}
const handleSubmit = () => {
  formRef.value.validate((valid) => {
    if (!valid) return

    formDrawerRef.value.showLoading()

    const fun = editId.value ? "修改方法" : "新建方法"
    //fun.then((res) => {})
    console.log("提交成功")
    formDrawerRef.value.close()
  }).finally(() => {
    formDrawerRef.value.hideLoading()
  })
}

const handleEdit = (row) => {
  editId.value = row.id
  form.name = row.name
  form.order = row.order
  formDrawerRef.value.open()
}

const handleDelete = (id) => {
  console.log(id)
  //删除方法
}
const emit = defineEmits(["change"])
//选中图库分类ID
const handleChangeActiveId = (id) => {
  activeId.value = id
  emit("change", id)
}

defineExpose({
  handleCreate
})
</script>

<template>
  <el-aside width="220px" class="image-aside" v-loading="loading">
    <div class="top">
      <AsideList :active="activeId == item.id" v-for="(item, index) in list" :key="index"
                 @edit="handleEdit(item)"
                 @delete="handleDelete(item.id)"
      @click="handleChangeActiveId(item.id)">
        {{ item.name }}
      </AsideList>
    </div>
    <div class="bottom">
      <el-pagination background layout="prev, next" :total="total" :current-page="currentPage" :page-size="limit"
                     @current-change="getData"/>
    </div>
  </el-aside>
  <FormDrawer ref="formDrawerRef" :title="drawerTitle" @submit="handleSubmit">
    <el-form :model="form" ref="formRef" :rules="rules" label-width="80px" :inline="false">
      <el-form-item label="分类名称" prop="name">
        <el-input v-model="form.name"></el-input>
      </el-form-item>
      <el-form-item label="排序" prop="order">
        <el-input-number v-model="form.order" :min="1" :max="1000"/>
      </el-form-item>
    </el-form>
  </FormDrawer>
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