import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import router from "@/router";
import { createPinia } from 'pinia'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

const app = createApp(App);
for(const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}
const pinia = createPinia();
app.use(ElementPlus, {
    locale: zhCn,
})
import 'virtual:windi.css'
import 'nprogress/nprogress.css'
import {store} from "@/store";
app.use(router)
app.use(store)
app.use(pinia)
app.mount('#app')
