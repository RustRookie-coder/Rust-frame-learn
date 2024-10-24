import { defineStore } from 'pinia'
import {invoke} from "@tauri-apps/api/core";

export const useAuthStore = defineStore('auth', {
    state: () => ({
        isAuthenticated: false,
        user: null as null | { name: string }
    }),
    actions: {
        async login(username: string, password: string) {
            const res = await invoke<boolean>('login', {username: username, password: password})
            // 模拟登录逻辑，通常这里会进行 API 请求
            if (res) {
                this.isAuthenticated = true
                this.user = { name: 'Admin'}
            } else {
                throw new Error('Invalid credentials')
            }
        },
        async logout() {
            this.isAuthenticated = false
            this.user = null
        }
    }
})