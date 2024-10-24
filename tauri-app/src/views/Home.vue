<template>
  <div>
    <el-menu default-active="1">
      <el-menu-item index="1" @click="logout">Logout</el-menu-item>
    </el-menu>
    <h1>Welcome, {{ authStore.user?.name }}</h1>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/store/auth'
import { useRouter } from 'vue-router'

const authStore = useAuthStore()
const router = useRouter()

const logout = () => {
  authStore.logout()
  router.push('/login')
}
</script>