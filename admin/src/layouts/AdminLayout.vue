<template>
  <div class="flex h-screen bg-gray-100">
    <!-- Sidebar -->
    <aside class="w-64 bg-gray-800 text-white flex flex-col">
      <AppSidebarMenu class="flex-1" />
      <div class="p-4 space-y-3">
        <div class="flex items-center justify-between text-sm">
          <span>Language</span>
          <el-select v-model="locale" size="small" style="width: 120px">
            <el-option label="English" value="en" />
            <el-option label="简体中文" value="zh-cn" />
          </el-select>
        </div>
        <button @click="handleLogout" class="w-full bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">Logout</button>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 p-10 overflow-auto">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterView, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import AppSidebarMenu from '@/components/AppSidebarMenu.vue'
import { storeToRefs } from 'pinia'
import { useLocaleStore } from '@/stores/locale'

const authStore = useAuthStore()
const router = useRouter()
const localeStore = useLocaleStore()
const { current: locale } = storeToRefs(localeStore)

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<style>
</style> 