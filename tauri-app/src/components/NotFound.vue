<script setup lang="ts">

</script>

<template>
  <div>
    <el-result icon="warning" title="404 tips" sub-title="Not Found">
      <template #extra>
        <el-button type="primary" @click="$router.push('/')">
          回到首页
        </el-button>
      </template>
    </el-result>
  </div>
</template>

<style scoped>

</style>