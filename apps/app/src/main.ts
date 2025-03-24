import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia())
app.use(router)

// 打印后端日志到控制台
//import { attachConsole } from '@tauri-apps/plugin-log'
//const detach = await attachConsole()
// call detach() if you do not want to print logs to the console anymore

app.mount('#app')
