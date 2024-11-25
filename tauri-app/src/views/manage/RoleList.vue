<script setup lang="ts">

import FormDrawer from "@/components/FormDrawer.vue";
import {useInitForm, useInitTable} from "@/common/init";
import {deleteRole, roleList, setRoleRules, updateRoleStatus} from "@/api/role";
import ListHeader from "@/components/ListHeader.vue";
import {ref} from "vue";
import {ruleList} from "@/api/rule";

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
const treeHeight = ref(0)
const roleId = ref(0)
const setRuleFormDrawerRef = ref(null)
const ruleArray = ref([])
const defaultExpandedKeys = ref([])
const elTreeRef = ref(null)
//current roles has ids
const ruleIds = ref([])
const checkStrictly = ref(false)
const handleSetRuleSubmit = () => {
  //@ts-ignore
  setRuleFormDrawerRef.value.showLoading()
  setRoleRules(roleId.value, ruleIds.value).then(res => {
    getData()
    //@ts-ignore
    setRuleFormDrawerRef.value.close()
  })
      .finally(() => {
        //@ts-ignore
        setRuleFormDrawerRef.value.hideLoading()
      })
}
const openSetRule = (row) => {
  roleId.value = row.id
  treeHeight.value = window.innerHeight - 180
  checkStrictly.value = true
  ruleList().then(res => {
    //@ts-ignore
    ruleArray.value = res.list
    //@ts-ignore
    defaultExpandedKeys.value = res.list.map(o => o.id)
    //@ts-ignore
    setRuleFormDrawerRef.value.open()
    //当前角色拥有的角色id
    ruleIds.value = row.rules.map(o => o.id)
    setTimeout(() => {
      //@ts-ignore
      elTreeRef.value.setCheckedKeys(ruleIds.value)
      checkStrictly.value = false
    }, 150)
  })
}
const handleTreeCheck = (...e) => {
  const {checkedKeys, halfCheckedKeys} = e[1]
  //@ts-ignore
  ruleIds.value = [...checkedKeys, ...halfCheckedKeys]
}
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
          <el-button type="primary" text size="default" @click="openSetRule(scope.row)">配置权限</el-button>
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
      <el-pagination background layout="prev, pager, next"
                     :total="total"
                     :current-page="currentPage"
                     :page-size="limit"
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
    <FormDrawer ref="setRuleFormDrawerRef" title="权限配置" @submit="handleSetRuleSubmit">
      <el-tree-v2 ref="elTreeRef" node-key="id"
                  :default-expanded-keys="defaultExpandedKeys"
                  :data="ruleArray"
                  :check-strictly="checkStrictly"
                  :props="{ label: 'name', children: 'child'}"
                  @check="handleTreeCheck"
                  show-checkbox :height="treeHeight">
        <template #default="{ node, data }">
          <div class="flex items-center">
            <el-tag :type="data.menu ? 'primary' : 'info'" size="small">
              {{ data.menu ? "菜单" : "权限" }}
            </el-tag>
            <span class="ml-2 text-sm">{{ data.name }}</span>
          </div>
        </template>
      </el-tree-v2>
    </FormDrawer>
  </el-card>
</template>
<style>
</style>