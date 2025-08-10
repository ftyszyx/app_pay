<template>
  <div class="flex flex-col h-full">
    <div class="text-2xl font-bold p-5 text-center">{{ $t('app.title') }}</div>
    <AppMenu :items="menuItems" mode="vertical" class="flex-1" />
  </div>
</template>

<script setup lang="ts">
import AppMenu from './AppMenu.vue'
import AppMenuSub from './AppMenuSub.vue'
import type { AdminMenuItem } from '@/types/menu'
import { RoutePath } from '@/types/route'



const menuItems: AdminMenuItem[] = [
  { label: 'menu.dashboard', icon: 'House', path: RoutePath.AdminDashboard },
  { label: 'menu.apps', icon: 'Cpu', path: RoutePath.AdminApps },
  { label: 'menu.products', icon: 'Goods', path: RoutePath.AdminProducts },
  { label: 'menu.orders', icon: 'Tickets', path: RoutePath.AdminOrders },
  { label: 'menu.users', icon: 'User', path: RoutePath.AdminUsers },
]
</script>

<style scoped>
</style>


