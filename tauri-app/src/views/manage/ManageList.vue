<script setup lang="ts">

import {Refresh} from "@element-plus/icons-vue";
import {computed, reactive, ref} from "vue";
import FormDrawer from "@/components/FormDrawer.vue";
import ChooseImage from "@/components/Image/ChooseImage.vue";
import {useInitForm, useInitTable} from "@/common/init";
import {deleteManager, manageList, updateManagerStatus} from "@/api/manager";

const {
  searchForm,
  tableData,
  total,
  currentPage,
  limit,
  getData,
} = useInitTable({
  searchForm: {
    keyword: ""
  },
  getList: manageList,
  onGetListSuccess: (data) => {
    tableData.value = data.map(o => {
      o.statusLoading = false
      return o
    })
    console.log(JSON.stringify(data))
  },
  delete: deleteManager,
  updateStatus: updateManagerStatus
})
//"超级管理员, 管理员, 普通用户, 游客"
const roles = ref([{
  id: 1,
  name: "超级管理员"
}, {
  id: 2,
  name: "管理员",
}, {
  id: 3,
  name: "普通用户"
}, {
  id: 4,
  name: "游客"
}])
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
    username: "",
    password: "",
    role_id: null,
    status: 1,
    avatar: "",
  },
  rules: {
    username: [{
      required: true,
      message: '用户名不能为空',
      trigger: 'blur'
    }],
    password: [{
      required: true,
      message: '密码不能为空',
      trigger: 'blur'
    }]
  },
  getData
})
</script>

<template>
  <el-card shadow="never" class="border-0">

    <el-form :model="searchForm" class="mb-3">
      <el-row :gutter="20">
        <el-col :span="8" :offset="0">
          <el-form-item label="关键词">
            <el-input v-model="searchForm.keyword" placeholder="管理员昵称" clearable></el-input>
          </el-form-item>
        </el-col>
        <el-col :span="8" :offset="8">
          <div class="flex items-center justify-end">
            <el-button type="primary" @click="getData">搜索</el-button>
            <el-button @click="resetForm">重置</el-button>
          </div>
        </el-col>
      </el-row>
    </el-form>


    <div class="flex items-center justify-between mb-4">
      <el-button type="primary" size="default" @click="handleCreate">新增</el-button>
      <el-tooltip class="box-item" effect="dark" content="" placement="top">
        <el-button text @click="getData">
          <el-icon :size="20">
            <refresh/>
          </el-icon>
        </el-button>
      </el-tooltip>
    </div>

    <el-table :data="tableData" stripe style="width: 100%">
      <el-table-column prop="username" label="管理员">
        <template #default="{ row }">
          <div class="flex items-center">
            <el-avatar :size="60" :src="row.avatar" @error="errorHandler">
              <img :src="row.avatar"/>
            </el-avatar>
            <div class="ml-3">
              <h6>{{ row.username }}</h6>
              <small>ID: {{ row.id }}</small>
            </div>
          </div>
        </template>
      </el-table-column>
      <el-table-column label="所属管理员" align="center">
        <template #default="{ row }">
          {{ row.role?.name || "-" }}
        </template>
      </el-table-column>

      <el-table-column></el-table-column>

      <el-table-column prop="create_time" label="状态">
        <template #default="{ row }">
          <el-switch v-model="row.status"
                     :active-value="1" :loading="row.statusLoading"
                     :disabled="row.isSuper == 1"
                     :inactive-value="0" @change="handleStatusChange($event, row)">
          </el-switch>
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center">
        <template #default="scope">
          <small v-if="scope.row.isSuper == 1" class="text-sm text-gray-500">暂无操作</small>
          <div v-else>
            <el-button type="primary" text size="default" @click="handleEdit(scope.row)">修改</el-button>
            <el-popconfirm title="是否要删除该管理员?"
                           confirm-button-text="确认"
                           cancel-button-text="取消"
                           @confirm="handleDelete(scope.row.id)">
              <template #reference>
                <el-button text type="primary" size="default">
                  删除
                </el-button>
              </template>
            </el-popconfirm>
          </div>
        </template>
      </el-table-column>
    </el-table>
    <div class="flex items-center justify-center mt-5">
      <el-pagination background layout="prev, pager, next" :total="total" :current-page="currentPage" :page-size="limit"
                     @current-change="getData"/>
    </div>
    <FormDrawer ref="drawerRef" :title="drawerTitle" @submit="handleSubmit">
      <el-form :model="form" ref="formRef" :rules="rules" label-width="80px" :inline="false">
        <el-form-item label="用户名" prop="username">
          <el-input v-model="form.username" placeholder="用户名"></el-input>
        </el-form-item>
        <el-form-item label="密码" prop="password">
          <el-input v-model="form.password" placeholder="密码" :rows="5"/>
        </el-form-item>
        <el-form-item label="头像" prop="avatar">
          <ChooseImage v-model="form.avatar"/>
        </el-form-item>
        <el-form-item label="所属角色" prop="role_id">
          <el-select v-model="form.role_id" value-key="" placeholder="选择所属管理员" clearable filterable @change="">
            <el-option v-for="item in roles" :key="item.id" :label="item.name" :value="item.id">
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch v-model="form.status" :active-value="1" :inactive-value="0">
          </el-switch>
        </el-form-item>
      </el-form>
    </FormDrawer>
  </el-card>
</template>
<style>
</style>