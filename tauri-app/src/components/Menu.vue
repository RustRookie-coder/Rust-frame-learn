<script setup lang="ts">
import {onBeforeRouteUpdate, useRoute, useRouter} from "vue-router";
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
//监听路由变化
onBeforeRouteUpdate((to, from) => {
  defaultActive.value = to.path
})

//const asideMenus = computed(() => store.state.menus)
const asideMenus = [{
  "name": "首页",
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
      "path": "/skus/list"
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
  },
  {"name": "订单管理",
    "icon": "ticket",
    "child": [
      {
        "name": "订单列表",
        "icon": "money",
        "path": "/order/list"
      }
    ]
  },
  {"name": "管理员管理",
    "icon": "management",
    "child": [
      {
        "name": "管理员管理",
        "icon": "coordinate",
        "path": "/manage/list"
      },
      {
        "name": "权限管理",
        "icon": "connection",
        "path": "/access/list"
      },
      {
        "name": "角色管理",
        "icon": "mug",
        "path": "/role/list"
      }
    ]
  },
  {"name": "图库管理",
    "icon": "camera",
    "child": [
      {
        "name": "图库列表",
        "icon": "picture-filled",
        "path": "/image/list"
      },
      {
        "name": "公告管理",
        "icon": "notification",
        "path": "/notice/list"
      }
    ]
  },
]
</script>

<template>
  <div class="b-menu" :style="{ width: $store.state.asideWidth }">
    <el-menu
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

        <el-menu-item v-else :index="item.path">
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