import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import GoodList from "@/views/goods/GoodList.vue";
import Login from "@/views/Login.vue";
import NotFound from "@/components/NotFound.vue";
import {useAuthStore} from "@/store/auth";
import TodoDemo from "@/components/TodoDemo.vue";
import {hideFullLoading, showFullLoading} from "@/utils/common";
import Panel from "@/layout/Panel.vue";
import CategoryList from "@/views/category/CategoryList.vue";
import UserList from "@/views/users/UserList.vue";
import Home from "@/views/Home.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'Home',
        component: Panel,
        meta: { requiresAuth: true, title: "首页" },
        children: [
            {
                path: '/',
                component: Home,
                meta: {
                    title: "主页"
                }
            },
            {
                path: '/goods/list',
                component: GoodList,
                meta: {
                    title: "商品管理"
                }
            },
            {
                path: '/category/list',
                component: CategoryList,
                meta: {
                    title: "分类列表"
                }
            },
            {
                path: '/users/list',
                component: UserList,
                meta: {
                    title: "用户管理"
                }
            }
        ]
    },
    {
        path: '/login',
        name: 'Login',
        component: Login,
        meta: {
            title: "登陆页"
        }
    },
    {
        path: '/todo',
        name: 'Todo',
        component: TodoDemo
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        component: NotFound
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.VITE_API_URL),
    routes,
})

let hasNewRoutes = false

// 全局路由守卫
router.beforeEach(async (to, from, next) => {
    await showFullLoading()
    const authStore = useAuthStore()
    if (to.meta.requiresAuth && !authStore.isAuthenticated) {
        // 如果没有登录，则跳转到登录页面
        next({ name: 'Login' })
    } else {
        next()  // 否则放行
    }
})

router.afterEach(async (to, from) => {
    await hideFullLoading();
})

export default router

const asyncRoutes  = [
    {
        path: '/',
        name: "/",
        component: '',
    },
    {
        path: '/goods/list',
        name: '/goods/list',
        component: GoodList,
        meta: {
            title: "商品管理"
        }
    },
    {
        path: '/category/list',
        name: '/category/list',
        component: CategoryList,
        meta: {
            title: "分类列表"
        }
    },
    {
        path: '/users/list',
        name: '/users/list',
        component: UserList,
        meta: {
            title: "用户管理"
        }
    }
]

export const addRoutes = (menus) => {
    let hasNewRoutes = false
    const findAndAddRoutesByMenus = (arr) => {
        arr.forEach(e => {
            let item = asyncRoutes.find(o => o.path == e.path)
            if(item && !router.hasRoute(item.path)) {
                route.addRoute("home", item)
                hasNewRoutes = true
            }
            if(e.child && e.child.length > 0) {
                findAndAddRoutesByMenus(e.child)
            }
        })
    }

    findAndAddRoutesByMenus(menus)

    return hasNewRoutes
}