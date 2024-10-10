import {createRouter, createWebHistory} from 'vue-router'


const routes = [
    {
        path: '/',
        component: () => import('@/views/login/index.vue'),
        meta: {
            requiresAuth: false,
        },
    },
    // 其他路由将在后续添加
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router
