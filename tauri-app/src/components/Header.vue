<script setup lang="ts">

import {Aim, ArrowDown, ElemeFilled, Expand, Fold, FullScreen, Refresh} from "@element-plus/icons-vue";
import {useRouter} from "vue-router";
import {useAuthStore} from "@/store/auth";
import {ElMessage, ElMessageBox} from "element-plus";
import { useFullscreen } from '@vueuse/core'
const { isFullscreen, toggle } = useFullscreen()
import {reactive, ref} from 'vue'
import FormDrawer from "@/components/FormDrawer.vue";
const formDrawerRef = ref(null)

const form = reactive({
  oldpassword: ref(''),
  password: ref(''),
  repassword: ref(''),
})
const handleCommand = (c) => {
  switch (c) {
    case "logout":
      logout()
      break;
    case "rePassword":
      formDrawerRef.value.open()
      break;
  }
}
const router = useRouter()
const authStore = useAuthStore()
const logout = () => {
  ElMessageBox.confirm(
      '是否确认登出?',
      'Warning',
      {
        confirmButtonText: '确认',
        cancelButtonText: '取消',
        type: 'warning',
      }
  )
      .then(() => {
        authStore.logout()
        router.push('/login')
        ElMessage({
          type: 'success',
          message: '退出成功',
        })
      })
      .catch(() => {
        ElMessage({
          type: 'info',
          message: '取消',
        })
      })
}

const handleRefresh = () => {
  location.reload()
}

const formRef = ref(null)
const onSubmit = async () => {
  formRef.value.validate((valid) => {
    if(valid) {
      formDrawerRef.value.showLoading()
      try {

      } catch (error) {
        console.log("error:" + error)
        alert('Login failed: ' + error.message)
      }
    } else {
      return false;
    }
  }).finally(() => {
    formDrawerRef.value.hideLoading()
  })
}

const rules = {
  oldpassword: [
    {
      required: true, message: '用户密码不能为空', trigger: 'blur'
    },
    {
      min:3, max: 15, message: '密码长度至少3 ~ 15 个字符', trigger: 'blur'
    }
  ],
  password: [{
    required: true, message: '用户密码不能为空', trigger: 'blur'
  }, {
    min: 5, max: 15, message: '密码长度至少5 ～ 15个字符', trigger: 'blur'
  }],
  repassword: [{
    required: true, message: '用户密码不能为空', trigger: 'blur'
  }, {
    min: 5, max: 15, message: '密码长度至少5 ～ 15个字符', trigger: 'blur'
  }],
}
</script>
<template>
  <div class="b-header">
    <span class="logo">
      <el-icon><eleme-filled/></el-icon>
      BI-Coding
    </span>
    <el-icon class="icon-btn" @click="$store.commit('handleAsideWidth')">
      <fold v-if="$store.state.asideWidth == '250px'"/>
      <Expand v-else/>
    </el-icon>
    <el-tooltip effect="dark" content="刷新" placement="bottom">
      <el-icon class="icon-btn" @click="handleRefresh">
        <refresh/>
      </el-icon>
    </el-tooltip>
    <div class="ml-auto flex items-center">
      <el-tooltip effect="dark" content="全屏" placement="bottom">
        <el-icon class="icon-btn" @click="toggle">
          <full-screen v-if="!isFullscreen"/>
          <aim v-else/>
        </el-icon>
      </el-tooltip>
      <el-dropdown class="dropdown" @command="handleCommand">
        <span class="flex items-center text-light-50">
          <el-avatar class="mr-2" :size="25"/>
          {{ authStore.user?.name }}
          <el-icon class="el-icon-right" :src="ArrowDown">
            <arrow-down/>
          </el-icon>
        </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="rePassword">修改密码</el-dropdown-item>
            <el-dropdown-item command="logout">退出登陆</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>

  <form-drawer ref="formDrawerRef" title="修改密码" destroyOnClose @submit="onSubmit">
        <el-form ref="formRef" :rules="rules" :model="form" label-width="100px" size="default">
          <el-form-item prop="oldpassword" label="旧密码">
            <el-input v-model="form.oldpassword" type="password" placeholder="请输入旧密码">
            </el-input>
          </el-form-item>
          <el-form-item prop="password" label="新密码">
            <el-input v-model="form.password" type="password" placeholder="请输入密码" show-password>
            </el-input>
          </el-form-item>
          <el-form-item prop="repassword" label="确认新密码">
            <el-input v-model="form.repassword" type="password" placeholder="请输入确认密码" show-password>
            </el-input>
          </el-form-item>
        </el-form>
  </form-drawer>
</template>

<style scoped>
.b-header {
  @apply flex items-center bg-indigo-500 text-light-50 fixed top-0 right-0 left-0;
  height: 64px;
  z-index: 1000;
}

.logo {
  width: 250px;
  @apply flex justify-center items-center text-xl font-thin;
}

.icon-btn {
  @apply flex justify-center items-center;
  width: 42px;
  height: 64px;
  cursor: pointer;
}

.icon-btn:hover {
  @apply bg-indigo-600;
}

.b-header .dropdown {
  height: 64px;
  cursor: pointer;
  @apply flex justify-center items-center mx-5;
}
</style>