<script lang="ts" setup>
import {ref} from 'vue'
import {ArrowDown} from "@element-plus/icons-vue";
import {onBeforeRouteUpdate, useRoute} from "vue-router";
import {useCookies} from "@vueuse/integrations/useCookies";
import router from "@/router";

const route = useRoute()
const activeTab = ref(route.path)
const cookie = useCookies()
const tabList = ref([
  {
    title: '后台首页',
    path: "/"
  },
])

const addTab = (tab) => {
  let noTab = tabList.value.findIndex(t => t.path == tab.path) == -1
  if (noTab) {
    tabList.value.push(tab)
  }
  cookie.set("tabList", tabList.value)
}

//初始化标签导航列表
const initTabList = () => {
  let tbs = cookie.get("tabList")
  if (tbs) {
    tabList.value = tbs
  }
}
initTabList()
onBeforeRouteUpdate((to, from) => {
  activeTab.value = to.path
  addTab({
    title: to.meta.title,
    path: to.path
  })
})

const changeTab = (path: string) => {
  console.log("change tab" + path);
  activeTab.value = path
  router.push(path).catch((error) => {
    console.error("Navigation failed:", error);
  });
}
const removeTab = (path: string) => {
  console.log("remove tab:" + path);
  const tabs = tabList.value
  let activeName = activeTab.value
  if (activeName === path) {
    tabs.forEach((tab, index) => {
      if (tab.path === path) {
        const nextTab = tabs[index + 1] || tabs[index - 1]
        if (nextTab) {
          activeName = nextTab.path
        }
      }
    })
  }

  activeTab.value = activeName
  tabList.value = tabs.filter((tab) => tab.path !== path)
  cookie.set("tabList", tabList.value)
}

const handleClose = (c) => {
  if (c == "clearAll") {
    //切换回首页
    activeTab.value = "/"
    //过滤只剩下首页
    tabList.value = [{
      title: '后台首页',
      path: "/"
    }]
  } else if (c == "clearOther") {
    tabList.value = tabList.value.filter(tab => tab.path == "/" || tab.path == activeTab.value)
  }
  cookie.set("tabList", tabList.value)
}
</script>
<template>
  <div class="b-tag-list" :style="{ left:$store.state.asideWidth }">
    <el-tabs
        v-model="activeTab"
        type="card"
        class="flex-1"
        @tab-remove="removeTab"
        style="min-width: 100px"
        @tab-change="changeTab"
    >
      <el-tab-pane
          v-for="item in tabList"
          :key="item.path"
          :label="item.title"
          :name="item.path"
          :closable="item.path != '/'"
      >
      </el-tab-pane>
    </el-tabs>

    <span class="tag-btn">
      <el-dropdown @command="handleClose">
    <el-icon><arrow-down/></el-icon>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item command="clearOther">关闭其他</el-dropdown-item>
        <el-dropdown-item command="clearAll">全部关闭</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
  </span>
  </div>
  <div style="height: 44px">

  </div>
</template>

<style>
.b-tag-list {
  @apply fixed bg-gray-100 flex items-center px-2;
  top: 64px;
  right: 0;
  height: 44px;
  z-index: 100;
}

.tag-btn {
  @apply bg-white rounded ml-auto flex items-center justify-center px-2;
  height: 32px;
}

.el-tabs__header {
  border: 0 !important;
  @apply mb-0;
}

:deep(.el-tabs__header) {
  border: 0 !important;
  @apply mb-0;
}

.el-tabs__nav {
  border: 0 !important;
}

:deep(.el-tabs__nav) {
  border: 0 !important;
}

.el-tabs__item {
  border: 0 !important;
  height: 32px;
  line-height: 32px;
  @apply bg-white mx-1 rounded;
}

:deep(.el-tabs__item) {
  border: 0 !important;
  height: 32px;
  line-height: 32px;
  @apply bg-white mx-1 rounded;
}

.el-tabs__nav-next, .el-tabs__nav-prev {
  line-height: 32px;
  height: 32px;
}

.is-disabled {
  cursor: not-allowed;
  @apply text-gray-300;
}
</style>