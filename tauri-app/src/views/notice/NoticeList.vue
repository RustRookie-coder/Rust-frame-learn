<script setup lang="ts">

import FormDrawer from "@/components/FormDrawer.vue";
import {useInitForm, useInitTable} from "@/common/init";
import {deleteNotice, noticeList} from "@/api/notice";
import ListHeader from "@/components/ListHeader.vue";

const {
  tableData,
  total,
  currentPage,
  limit,
  handleDelete,
  getData,
} = useInitTable({
  searchForm: {
    keyword: ""
  },
  getList: noticeList,
  onGetListSuccess: (data) => {
    tableData.value = data.map(o => {
      o.statusLoading = false
      return o
    })
    console.log(JSON.stringify(data))
  },
  delete: deleteNotice
})
// delete notification by backend
const {
  drawerRef,
  formRef,
  form,
  editId,
  drawerTitle,
  rules,
  handleSubmit,
  handleEdit,
  handleCreate,
} = useInitForm({
  form: {
    title: "",
    content: "",
  },
  rules: {
    title: [{
      required: true,
      message: '公告标题不能为空',
      trigger: 'blur'
    }],
    content: [{
      required: true,
      message: '公告内容不能为空',
      trigger: 'blur'
    }]
  },
  getData
})
</script>

<template>
  <el-card shadow="never" class="border-0">
    <ListHeader @create="handleCreate" @refresh="getData"/>
    <el-table :data="tableData" stripe style="width: 100%">
      <el-table-column prop="title" label="公告标题"></el-table-column>
      <el-table-column prop="create_time" label="发布时间"></el-table-column>
      <el-table-column label="操作" align="center">
        <template #default="scope">
          <el-button type="primary" text size="default" @click="handleEdit(scope.row)">修改</el-button>
          <el-popconfirm title="是否要删除该公告?"
                         confirm-button-text="确认"
                         cancel-button-text="取消"
                         @confirm="handleDelete(scope.row.id)">
            <template #reference>
              <el-button text type="primary" size="default">
                删除
              </el-button>
            </template>
          </el-popconfirm>
        </template>
      </el-table-column>
    </el-table>
    <div class="flex items-center justify-center mt-5">
      <el-pagination background layout="prev, pager, next" :total="total" :current-page="currentPage" :page-size="limit"
                     @current-change="getData"/>
    </div>
    <FormDrawer ref="drawerRef" :title="drawerTitle" @submit="handleSubmit">
      <el-form :model="form" ref="formRef" :rules="rules" label-width="80px" :inline="false">
        <el-form-item label="公告标题" prop="title">
          <el-input v-model="form.title" placeholder="公告标题"></el-input>
        </el-form-item>
        <el-form-item label="公告内容" prop="content">
          <el-input v-model="form.content" placeholder="公告内容" type="textarea" :rows="5"/>
        </el-form-item>
      </el-form>
    </FormDrawer>
  </el-card>
</template>
<style>
</style>