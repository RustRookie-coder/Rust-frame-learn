<script setup lang="ts">

import Header from "@/components/Header.vue";
import Menu from "@/components/Menu.vue";
import TagList from "@/components/TagList.vue";
</script>

<template>
  <el-container>
    <el-header>
      <Header/>
    </el-header>
    <el-container>
      <el-aside>
        <Menu></Menu>
      </el-aside>
      <el-main>
        <TagList></TagList>
        <router-view></router-view>
      </el-main>
    </el-container>
  </el-container>
</template>