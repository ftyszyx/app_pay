<template>
  <div class="flex h-screen bg-gray-100">
    <!-- Sidebar -->
    <aside class="w-64 bg-gray-800 text-white flex flex-col">
      <div class="text-2xl font-bold p-5 text-center">AppPay Admin</div>
      <nav class="flex-grow">
        <RouterLink to="/admin/dashboard" class="block py-2.5 px-4 rounded transition duration-200 hover:bg-gray-700">Dashboard</RouterLink>
        <RouterLink to="/admin/products" class="block py-2.5 px-4 rounded transition duration-200 hover:bg-gray-700">Products</RouterLink>
        <RouterLink to="/admin/orders" class="block py-2.5 px-4 rounded transition duration-200 hover:bg-gray-700">Orders</RouterLink>
        <RouterLink to="/admin/users" class="block py-2.5 px-4 rounded transition duration-200 hover:bg-gray-700">Users</RouterLink>
      </nav>
      <div class="p-4">
        <button @click="handleLogout" class="w-full bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">
          Logout
        </button>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 p-10 overflow-auto">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterLink, RouterView, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const router = useRouter()

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<style>
</style> 