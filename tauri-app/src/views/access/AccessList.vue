<script setup lang="ts">

import ListHeader from "@/components/ListHeader.vue";
import {useInitForm, useInitTable} from "@/common/init";
import {createRule, ruleList, updateRule} from "@/api/rule";
import {ref} from "vue";
import FormDrawer from "@/components/FormDrawer.vue";

const defaultExpandedKeys = ref([])
const rules = ref([])
const {
  tableData,
  getData
} = useInitTable({
  getList: ruleList,
  onGetListSuccess: (data) => {
    tableData.value = data.list
    defaultExpandedKeys.value = data.list.map(o => o.id)
  }
})

const {
  drawerRef,
  drawerTitle,
  form,
  handleCreate,
  handleEdit,
  handleSubmit
} = useInitForm({
  form: {
    rule_id: 0,
    menu: 0,
    name: "",
    condition: "",
    method: "GET",
    status: 1,
    order: 50,
    icon: "",
    path: ""
  },
  rules: [

  ],
  getData,
  update: updateRule,
  create: createRule
})

</script>

<template>
  <el-card shadow="never" class="border-0">
    <ListHeader @create="handleCreate" @refresh="getData" />
    <el-tree :data="tableData" :props="{ label:'name', children:'child' }"
    :default-expanded-keys="defaultExpandedKeys">
      <template #default="{ node, data }">
        <div class="custom-tree-node">
          <el-tag :type="data.menu ? 'primary' : 'info'">{{ data.menu ? "菜单" : "权限" }}</el-tag>
          <el-icon v-if="data.icon" :size="16" class="ml-2"><component :is="data.icon"/></el-icon>
          <span>{{ data.name }}</span>

          <div class="ml-auto">
            <el-switch :modelValue="data.status" :active-value="1" :inactive-value="0" />
            <el-button text type="primary" @click.stop="handleEdit(data)">修改</el-button>
            <el-button text type="primary">增加</el-button>
            <el-button text type="primary">修改</el-button>
          </div>
        </div>
      </template>
    </el-tree>

    <FormDrawer ref="drawerRef" :title="drawerTitle" @submit="handleSubmit">
      <el-form :model="form" ref="formRef" label-width="80px" :inline="false">
        <el-form-item label="上级菜单" prop="rule_id">
          <el-input v-model="form.rule_id"></el-input>
        </el-form-item>
        <el-form-item label="菜单/规则" prop="menu">
          <el-input v-model="form.menu" placeholder=""/>
        </el-form-item>
        <el-form-item label="菜单/权限名称" prop="name">
          <el-input v-model="form.name"/>
        </el-form-item>
        <el-form-item label="菜单图标" prop="icon">
          <el-input v-model="form.icon"/>
        </el-form-item>
        <el-form-item label="前端路由" prop="path">
          <el-input v-model="form.path"/>
        </el-form-item>
        <el-form-item label="后端规则" prop="condition">
          <el-input v-model="form.condition"/>
        </el-form-item>
        <el-form-item label="请求方式" prop="method">
          <el-input v-model="form.method"/>
        </el-form-item>
        <el-form-item label="排序" prop="order">
          <el-input v-model="form.order"/>
        </el-form-item>
      </el-form>
    </FormDrawer>
  </el-card>

</template>

<style>
.custom-tree-node {
  flex: 1;
  display: flex;
  align-items: center;
  font-size: 14px;
  padding-right: 8px;
}
.el-tree-node__content {
  padding: 20px 0;
}
</style>