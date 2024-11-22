<script setup lang="ts">

import ListHeader from "@/components/ListHeader.vue";
import {useInitForm, useInitTable} from "@/common/init";
import {createRule, deleteRule, ruleList, updateRule, updateRuleStatus} from "@/api/rule";
import {ref} from "vue";
import FormDrawer from "@/components/FormDrawer.vue";
import IconSelect from "@/components/IconSelect.vue";

const defaultExpandedKeys = ref([])
const options = ref([])
const {
  tableData,
  getData,
  handleStatusChange,
  handleDelete,
} = useInitTable({
  getList: ruleList,
  onGetListSuccess: (data) => {
    tableData.value = data.list
    options.value = data.list
    defaultExpandedKeys.value = data.list.map(o => o.id)
  },
  delete: deleteRule,
  update: updateRuleStatus
})

const {
  drawerRef,
  drawerTitle,
  form,
  rules,
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
  rules: [],
  getData,
  update: updateRule,
  create: createRule
})

const addChild = (id) => {
  handleCreate()
  form.rule_id = id
  form.status = 1
}
</script>

<template>
  <el-card shadow="never" class="border-0">
    <ListHeader @create="handleCreate" @refresh="getData"/>
    <el-tree :data="tableData" :props="{ label:'name', children:'child' }"
             :default-expanded-keys="defaultExpandedKeys">
      <template #default="{ node, data }">
        <div class="custom-tree-node">
          <el-tag :type="data.menu ? 'primary' : 'info'">{{ data.menu ? "菜单" : "权限" }}</el-tag>
          <el-icon v-if="data.icon" :size="16" class="ml-2">
            <component :is="data.icon"/>
          </el-icon>
          <span>{{ data.name }}</span>

          <div class="ml-auto">
            <el-switch :modelValue="data.status"
                       :active-value="1"
                       :inactive-value="0"
                       @change="handleStatusChange($event, data)"/>
            <el-button text type="primary" @click.stop="handleEdit(data)">修改</el-button>
            <el-button text type="primary" @click.stop="addChild(data.id)">增加</el-button>
            <el-popconfirm confirm-button-text="确认"
                           cancel-button-text="取消"
                           title="是否要删除该记录"
                           @confirm="handleDelete(data.id)">
              <template #reference>
                <el-button text type="primary">删除</el-button>
              </template>
            </el-popconfirm>
          </div>
        </div>
      </template>
    </el-tree>

    <FormDrawer ref="drawerRef" :rules="rules" :title="drawerTitle" @submit="handleSubmit">
      <el-form :model="form" ref="formRef" label-width="80px" :inline="false">
        <el-form-item label="上级菜单" prop="rule_id">
          <el-cascader v-model="form.rule_id"
                       :options="options"
                       :props="{ value: 'id', label:'name', children:'child', checkStrictly:true }"
                       placeholder="请选择上级菜单"
          />
        </el-form-item>
        <el-form-item label="菜单/规则" prop="menu">
          <el-radio-group v-model="form.menu">
            <el-radio :value="1" border>菜单</el-radio>
            <el-radio :value="0" border>规则</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="名称" prop="name">
          <el-input v-model="form.name" style="width: 30%" placeholder="名称"/>
        </el-form-item>
        <el-form-item label="菜单图标" prop="icon" v-if="form.menu == 1">
          <IconSelect v-model="form.icon" style="width: 35%"/>
        </el-form-item>
        <el-form-item label="前端路由" prop="path" style="width: 80%" v-if="form.menu == 1 && form.rule_id > 0">
          <el-input v-model="form.path"/>
        </el-form-item>
        <el-form-item label="后端规则" prop="condition" style="width: 80%" v-if="form.menu == 0">
          <el-input v-model="form.condition" placeholder="后端规则"/>
        </el-form-item>
        <el-form-item label="请求方式" prop="method" v-if="form.menu == 0">
          <el-select v-model="form.method" placeholder="请选择请求方式" style="width: 35%">
            <el-option
                v-for="item in ['GET', 'POST', 'PUT', 'DELETE']"
                :key="item"
                :label="item"
                :value="item"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="排序" prop="order">
          <el-input-number v-model="form.order" :min="0" :max="1000"/>
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