import './assets/main_output.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import 'element-plus/dist/index.css'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
