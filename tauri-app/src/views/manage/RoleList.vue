<script setup lang="ts">

import FormDrawer from "@/components/FormDrawer.vue";
import {useInitForm, useInitTable} from "@/common/init";
import {deleteRole, roleList, updateRoleStatus} from "@/api/role";
import ListHeader from "@/components/ListHeader.vue";

const {
  tableData,
  total,
  currentPage,
  limit,
  handleDelete,
  getData,
  handleStatusChange,
} = useInitTable({
  searchForm: {
    keyword: ""
  },
  getList: roleList,
  onGetListSuccess: (data) => {
    tableData.value = data
    console.log(JSON.stringify(data))
  },
  delete: deleteRole,
  update: updateRoleStatus
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
    name: "",
    desc: "",
    status: 1
  },
  rules: {
    name: [{
      required: true,
      message: '名称不能为空',
      trigger: 'blur'
    }],
  },
  getData
})
</script>

<template>
  <el-card shadow="never" class="border-0">
    <ListHeader @create="handleCreate" @refresh="getData"/>
    <el-table :data="tableData" stripe style="width: 100%">
      <el-table-column prop="name" label="角色名称"></el-table-column>
      <el-table-column prop="desc" label="角色描述"></el-table-column>
      <el-table-column prop="status" label="状态">
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
        <el-form-item label="角色名称" prop="name">
          <el-input v-model="form.name" placeholder="角色名称"></el-input>
        </el-form-item>
        <el-form-item label="角色描述" prop="desc">
          <el-input v-model="form.desc" placeholder="角色描述" type="textarea" :rows="5"/>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch :model-value="form.status" :active-value="1" :inactive-value="0">
          </el-switch>
        </el-form-item>
      </el-form>
    </FormDrawer>
  </el-card>
</template>
<style>
</style>