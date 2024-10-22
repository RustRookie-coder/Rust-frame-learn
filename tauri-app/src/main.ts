import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import i18n from "./plugins/i18n";
import vuetify from "./plugins/vuetify";
import { router } from './router';
import store from "@/store";

const app = createApp(App);
app.use(i18n);
app.use(vuetify);
app.use(router);
app.use(store);
app.mount('#app')
