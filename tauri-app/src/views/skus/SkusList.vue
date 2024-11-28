<script setup lang="ts">

import FormDrawer from "@/components/FormDrawer.vue";
import {useInitForm, useInitTable} from "@/common/init";
import ListHeader from "@/components/ListHeader.vue";
import {createSkus, deleteSkus, getSkusList, updateSkus, updateSkusStatus} from "@/api/skus";
import TagInputs from "@/components/TagInputs.vue";
import {ref} from "vue";
import {ElNotification} from "element-plus";

const {
  tableData,
  total,
  currentPage,
  limit,
  loading,
  multipleTableRef,
  handleDelete,
  getData,
  handleStatusChange,
  handleSelectionChange,
  handleMultiDelete,
} = useInitTable({
  searchForm: {
    keyword: ""
  },
  getList: getSkusList,
  onGetListSuccess: (data) => {
    tableData.value = data
    console.log(JSON.stringify(data))
  },
  delete: deleteSkus,
  update: updateSkus,
  updateStatus: updateSkusStatus
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
    status: 1,
    default: "",
    order: 50
  },
  rules: {
    name: [{
      required: true,
      message: '规格名称不能为空',
      trigger: 'blur'
    }],
    default: [{
      required: true,
      message: '规格值不能为空',
      trigger: 'blur'
    }],
  },
  getData,
  update: updateSkus,
  create: createSkus,
  delete: deleteSkus,
})

</script>

<template>
  <el-card shadow="never" class="border-0">
    <ListHeader layout="create, delete, refresh" @create="handleCreate" @refresh="getData" @delete="handleMultiDelete"/>
    <el-table :data="tableData" stripe style="width: 100%"
              @selection-change="handleSelectionChange"
              ref="multipleTableRef"
              v-loading="loading">
      <el-table-column type="selection" width="55"/>
      <el-table-column prop="name" label="规格名称"/>
      <el-table-column prop="default" label="规格值" width="380"/>
      <el-table-column prop="order" label="排序"/>
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
          <el-button type="primary" text @click="handleEdit(scope.row)">修改</el-button>
          <el-popconfirm title="是否要删除规格?"
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
      <el-pagination background layout="prev, pager, next"
                     :total="total"
                     :current-page="currentPage"
                     :page-size="limit"
                     @current-change="getData"/>
    </div>
    <FormDrawer ref="drawerRef" :title="drawerTitle" @submit="handleSubmit" destroyOnClose>
      <el-form :model="form" ref="formRef" :rules="rules" label-width="80px" :inline="false">
        <el-form-item label="规格名称" prop="name">
          <el-input v-model="form.name" placeholder="规格名称"></el-input>
        </el-form-item>
        <el-form-item label="排序" prop="order">
          <el-input-number v-model="form.order" :min="0" :max="1000"></el-input-number>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch :model-value="form.status" :active-value="1" :inactive-value="0">
          </el-switch>
        </el-form-item>
        <el-form-item label="规格值" prop="default">
          <TagInputs v-model="form.default"/>
        </el-form-item>
      </el-form>
    </FormDrawer>
  </el-card>
</template>
<style>
</style>