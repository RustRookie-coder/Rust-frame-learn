import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import GoodList from "@/views/goods/GoodList.vue";
import Login from "@/views/Login.vue";
import NotFound from "@/components/NotFound.vue";
import {useAuthStore} from "@/store/auth";
import TauriDemo from "@/components/TauriDemo.vue";
import {hideFullLoading, showFullLoading} from "@/utils/common";
import Panel from "@/layout/Panel.vue";
import CategoryList from "@/views/category/CategoryList.vue";
import UserList from "@/views/users/UserList.vue";
import Home from "@/views/Home.vue";
import OrderList from "@/views/order/OrderList.vue";
import CommentList from "@/views/comment/CommentList.vue";
import ImageList from "@/views/image/ImageList.vue";
import NoticeList from "@/views/notice/NoticeList.vue";
import SettingBase from "@/views/setting/SettingBase.vue";
import CouponList from "@/views/coupon/CouponList.vue";
import {useCookies} from "@vueuse/integrations/useCookies";
import Management from "@/views/management/Management.vue";

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
            },
            {
                path: "/order/list",
                name: "/order/list",
                component: OrderList,
                meta:{
                    title: "订单",
                }
            },
            {
                path: "/management",
                name: "/management",
                component: Management,
                meta: {
                    title: "管理员管理"
                }
            },
            {
                path: "/comment/list",
                name: "/comment/list",
                component: CommentList,
                meta: {
                    title: "评价",
                }
            },
            {
                path: "/image/list",
                name: "/image/list",
                component: ImageList,
                meta: {
                    title: "图库",
                }
            },
            {
                path: "/notice/list",
                name: "/notice/list",
                component: NoticeList,
                meta: {
                    title: "公告",
                }

            },
            {
                path: "/setting/base",
                name: "/setting/base",
                component: SettingBase,
                meta: {
                    title: "配置",
                }
            },
            {
                path: "/coupon/list",
                name: "/coupon/list",
                component: CouponList,
                meta: {
                    title: "优惠券",
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
        component: TauriDemo
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
    const cookie = useCookies()
    const token:string = cookie.get("token");
    authStore.isAuthenticated = cookie.get("auth")
    if (to.meta.requiresAuth && !authStore.isAuthenticated && token == null) {
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
    },
    {
        path: "/order/list",
        name: "/order/list",
        component: OrderList,
        meta:{
            title: "订单",
        }
    },
    {
        path: "/comment/list",
        name: "/comment/list",
        component: CommentList,
        meta: {
            title: "评价",
        }
    },
    {
        path: "/image/list",
        name: "/image/list",
        component: ImageList,
        meta: {
            title: "图库",
        }
    },
    {
        path: "/notice/list",
        name: "/notice/list",
        component: NoticeList,
        meta: {
            title: "公告",
        }

    },
    {
        path: "/setting/base",
        name: "/setting/base",
        component: SettingBase,
        meta: {
            title: "配置",
        }
    },
    {
        path: "/coupon/list",
        name: "/coupon/list",
        component: CouponList,
        meta: {
            title: "优惠券",
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