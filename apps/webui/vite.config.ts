import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
//import AutoImport from 'unplugin-auto-import/vite'
//import Components from 'unplugin-vue-components/vite'
//import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

import vueDevTools from 'vite-plugin-vue-devtools'

const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
//    AutoImport({
//      resolvers: [ElementPlusResolver()],
//    }),
//    Components({
//      resolvers: [ElementPlusResolver()],
//    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  css: {
    preprocessorOptions: {
      scss: { api: 'modern-compiler' },
    }
  },
  clearScreen: false,
  server: {
    port: 57350,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: 'ws',
        host,
        port: 57351,
      }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
      usePolling: true
    },
  },
  build: {
    outDir: '../../dist',
  },
})

