<template>
  <div>
    <h1>Welcome, {{ authStore.user?.name }}</h1>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/store/auth'
const authStore = useAuthStore()
</script>