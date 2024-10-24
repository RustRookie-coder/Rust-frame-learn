import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import Home from "@/views/Home.vue";
import Login from "@/views/Login.vue";
import NotFound from "@/components/NotFound.vue";
import {useAuthStore} from "@/store/auth";
import TodoDemo from "@/components/TodoDemo.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'Home',
        component: Home,
        meta: { requiresAuth: true }
    },
    {
        path: '/login',
        name: 'Login',
        component: Login
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

// 全局路由守卫
router.beforeEach((to, from, next) => {
    const authStore = useAuthStore()
    if (to.meta.requiresAuth && !authStore.isAuthenticated) {
        // 如果没有登录，则跳转到登录页面
        next({ name: 'Login' })
    } else {
        next()  // 否则放行
    }
})

export default router