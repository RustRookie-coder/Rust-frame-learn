import { defineStore } from 'pinia'
import {invoke} from "@tauri-apps/api/core";
import router from "@/router";
import {ElNotification} from "element-plus";
import { useCookies } from "@vueuse/integrations/useCookies";

export const useAuthStore = defineStore('auth', {
    state: () => ({
        isAuthenticated: false,
        user: null as null | { name: string }
    }),
    actions: {
        async login(username: string, password: string) {
            const cookie = useCookies()
            const res = await invoke<string>('login_command', {username: username, password: password})
            // 模拟登录逻辑，通常这里会进行 API 请求
            if (res.token.length > 0) {
                this.isAuthenticated = true
                this.user = { name: 'Admin'}
                cookie.set("token", res.token, { expires: new Date(Date.now() + 7 * 864e5), path: '/'})
                cookie.set("auth", this.isAuthenticated, { expires: new Date(Date.now() + 7 * 864e5), path: '/'})
                await router.push('/')
            } else {
                ElNotification({
                    message: '请求失败',
                    type: 'error',
                    duration: 3000
                })
            }
        },
        async logout() {
            const cookie = useCookies()
            this.isAuthenticated = false
            this.user = null
            cookie.remove("token")
        }
    }
})