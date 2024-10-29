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
      <el-aside :width="$store.state.asideWidth">
        <Menu></Menu>
      </el-aside>
      <el-main>
        <TagList></TagList>
        <router-view v-slot="{ Component }">
          <transition name="fade">
            <keep-alive :max="10">
              <component :is="Component"></component>
            </keep-alive>
          </transition>
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<style>
.el-aside {
  transition: all 0.2s;
}
</style>