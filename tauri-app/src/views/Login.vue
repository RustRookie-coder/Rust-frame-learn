<template>
  <el-row class="login-container">
    <el-col :lg="16" :md="12" class="left">
      <div>
        <div>Welcome</div>
        <div>This is vue demo web address</div>
        <el-header>Todo List</el-header>
        <el-form :inline="true" @submit.prevent="addTodo">
          <el-form-item>
            <el-input v-model="newTodo" placeholder="Enter a new todo..."/>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="addTodo">Add Todo</el-button>
          </el-form-item>
          <el-form-item>
            <el-button @click="(event) => warn('Form cannot be submitted yet.', event)">Submit</el-button>
          </el-form-item>
        </el-form>
        <el-col>
          <el-collapse v-for="(todo, index) in todos" :key="index">{{ todo }}</el-collapse>
        </el-col>
      </div>
    </el-col>
    <el-col :lg="8" :md="12" class="right">
      <h2 class="title">Welcome back</h2>
      <div>
        <span class="line"></span>
        <span>账号密码登陆</span>
        <span class="line"></span>
      </div>
      <el-form ref="formRef" :rules="rules" :model="form" class="w-[250px]">
        <el-form-item prop="username">
          <el-input v-model="form.username" placeholder="Username">
            <template #prefix>
              <el-icon><user/></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" placeholder="Password" show-password>
            <template #prefix>
              <el-icon><lock/></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-button round color="#626aef" class="w-[250px]" type="primary" :loading="loading" @click="onSubmit">Login</el-button>
        </el-form-item>
      </el-form>
    </el-col>
  </el-row>
</template>

<script setup lang="ts">
import {reactive, ref, warn, onMounted, onBeforeMount} from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/store/auth'
import { invoke } from "@tauri-apps/api/core"

const form = reactive({
  username: ref(''),
  password: ref('')
})
const router = useRouter()
const authStore = useAuthStore()
const loading = ref(false)

const onSubmit = async () => {
  //@ts-ignore
  formRef.value.validate((valid) => {
    if(valid) {
      loading.value = true
      try {
        authStore.login(form.username, form.password)
      } catch (error) {
        console.log("error:" + error)
        alert('Login failed: ' + error.message)
      }
    } else {
      return false;
    }
  }).finally(() => {
    loading.value = false
  })
}

const rules = {
  username: [
    {
      required: true, message: '用户名不能为空', trigger: 'blur'
    },
    {
      min:3, max: 5, message: '用户名长度至少3 - 5 个字符', trigger: 'blur'
    }
  ],
  password: [{
    required: true, message: '用户密码不能为空', trigger: 'blur'
  }, {
    min: 5, max: 15, message: '密码长度至少5 ～ 15个字符', trigger: 'blur'
  }],
}
//dishaxy.dishait.cn/shopadminapi
const formRef = ref(null)
const onKeyUp = async (e) => {
  if(e.key == "Enter") {
    await onSubmit()
  }
}


// 待办事项列表和输入的待办事项
const todos = ref<string[]>([]);
const newTodo = ref<string>('');

// 添加新的待办事项
const addTodo = async () => {
  if (newTodo.value) {
    const todo = await invoke<string>('add_todo', {todo: newTodo.value});
    todos.value.push(todo);
    newTodo.value = '';
  }
};
// 页面加载时获取待办事项
onMounted(async () => {
  todos.value = await invoke<string[]>('get_todos');
  document.addEventListener("keyup", onKeyUp);
});

onBeforeMount(async () => {
  document.removeEventListener("keyup", onKeyUp);
})

const warn = async (message, event) => {
  // 这里可以访问原生事件
  if (event) {
    event.preventDefault()
  }
  alert(message)
}
</script>
<style>
.login-container {
  @apply min-h-screen bg-indigo-500;
}
.login-container .left, .login-container .right {
  @apply flex items-center justify-center;
}

.login-container .right {
  @apply flex items-center justify-center bg-light-50 flex-col;
}
.left>div>div:first-child {
  @apply font-bold text-5xl text-light-50 mb4;
}
.left>div>div:last-child {
  @apply text-gray-200 text-sm;
}
.right .title{
  @apply font-bold text-3xl text-gray-800;
}
.right>div {
  @apply  flex items-center justify-center my-5 text-gray-300 space-x-2;
}
.right .line {
  @apply h-[1px] w-16 bg-gray-200;
}
</style>