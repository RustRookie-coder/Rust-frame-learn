<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {computed, ref} from "vue";
import {useStore} from "vuex";

const router = useRouter()
const handleSelect = (e) => {
  router.push(e)
}

const store = useStore();

const isCollapse = computed(() => !(store.state.asideWidth == '250px'))

const route = useRoute()
//默认选中
const defaultActive = ref(route.path)

//const asideMenus = computed(() => store.state.menus)
const asideMenus = [{
  "name": "后台面板",
  "icon": "help",
  "child": [{
    "name": "主控台",
    "icon": "home-filled",
    "path": "/"
  }]
},
  {
    "name": "商城管理",
    "icon": "shopping-bag",
    "child": [{
      "name": "商品管理",
      "icon": "shopping-cart-full",
      "path": "/goods/list"
    },{
      "name": "分类管理",
      "icon": "menu",
      "path": "/category/list"
    },{
      "name": "规格管理",
      "icon": "aim",
      "path": "/spec/list"
    }, {
      "name": "优惠券管理",
      "icon": "postcard",
      "path": "/coupon/list"
    }
    ]
  },
  {
    "name": "用户管理",
    "icon": "user-filled",
    "child": [{
      "name": "用户列表",
      "icon": "avatar",
      "path": "/users/list"
    }]
  }
]
</script>

<template>
  <div class="b-menu" :style="{ width: $store.state.asideWidth }">
    <el-menu
        default-active="2"
        class="border-0"
        @select="handleSelect"
        :collapse="isCollapse"
        :collapse-transition="false"
        unique-opened
        :default-active="defaultActive"
    >
      <template v-for="(item, index) in asideMenus" :key="index">
        <el-sub-menu v-if="item.child && item.child.length > 0" :index="item.name">
          <template #title>
            <el-icon>
              <component :is="item.icon"></component>
            </el-icon>
            <span>{{ item.name }}</span>
          </template>
          <el-menu-item v-for="(item2, index2) in item.child" :key=index2 :index="item2.path">
            <el-icon>
              <component :is="item2.icon"></component>
            </el-icon>
            <span>{{ item2.name }}</span>
          </el-menu-item>
        </el-sub-menu>

        <el-menu-item v-else :index="item.frontpath">
          <el-icon>
            <component :is="item.icon"></component>
          </el-icon>
          <span>{{ item.name }}</span>
        </el-menu-item>
      </template>
    </el-menu>
  </div>
</template>

<style>
.b-menu {
  transition: all 0.2s;
  top: 64px;
  bottom: 0;
  left: 0;
  overflow-y: auto;
  overflow-x: hidden;
  @apply shadow-md fixed bg-light-50;
}
.b-menu::-webkit-scrollbar {
  width: 0;
}
</style>