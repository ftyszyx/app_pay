<template>
  <div class="flex flex-col h-full">
    <div class="text-2xl font-bold p-5 text-center">AppPay Admin</div>
    <AppMenu :items="menuItems" mode="vertical" class="flex-1" />
  </div>
</template>

<script setup lang="ts">
import AppMenu from './AppMenu.vue'
import type { AdminMenuItem } from '@/types/menu'
import { RoutePath } from '@/types/route'



const menuItems: AdminMenuItem[] = [
  { label: 'Dashboard', icon: 'House', path: RoutePath.AdminDashboard },
  { label: 'Apps', icon: 'Cpu', path: RoutePath.AdminApps },
  { label: 'Products', icon: 'Goods', path: RoutePath.AdminProducts },
  { label: 'Orders', icon: 'Tickets', path: RoutePath.AdminOrders },
  { label: 'Users', icon: 'User', path: RoutePath.AdminUsers },
]
</script>

<style scoped>
</style>


