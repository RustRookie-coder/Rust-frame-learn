import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import VueDevTools from 'vite-plugin-vue-devtools';
import { fileURLToPath, URL } from 'url';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue(),
        VueDevTools(),
    ],
    server: {
        host: true,
        port: 3000
        // proxy: {
        //   '/api': {
        //     target: '',
        //     changeOrigin: true,
        //     rewrite: (path) => path.replace(/^\/api/, '/api/v1')
        //   }
        // }
    },
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        }
    },
})
