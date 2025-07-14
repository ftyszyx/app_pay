import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'
import components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import AutoImport from 'unplugin-auto-import/vite'
import path from 'node:path'

// https://vite.dev/config/
export default defineConfig(({mode})=>{
  const env=loadEnv(mode,path.resolve(__dirname,'env'))
  console.log("build with env",mode,env,process.cwd())
  return {
  plugins: [vue(),AutoImport({
    resolvers: [ElementPlusResolver({
      importStyle: false,
    })],
  }),components({
    resolvers: [ElementPlusResolver({
      importStyle: false,
    })],
  })],
  envDir:'./env',
    server: {
      port: 9070,
      proxy: {
        '/api': {
          target: 'http://localhost:3000',
          changeOrigin: true,
        },
      },
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
    }
  }
})
