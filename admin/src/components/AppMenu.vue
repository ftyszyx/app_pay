<template>
  <el-menu
    :default-active="activeIndex"
    class="h-full border-0"
    :mode="mode"
    :router="true"
  >
    <template v-for="(item, idx) in items" :key="item.index ?? item.path ?? String(idx)">
      <el-sub-menu v-if="item.children && item.children.length" :index="item.index ?? String(idx)">
        <template #title>
          <el-icon v-if="item.icon" class="mr-2">
            <component :is="item.icon" />
          </el-icon>
          <span>{{ item.label }}</span>
        </template>
        <app-menu-sub :items="item.children" />
      </el-sub-menu>
      <el-menu-item v-else :index="item.index ?? (item.path ?? String(idx))" :route="item.path" :disabled="item.disabled">
        <el-icon v-if="item.icon" class="mr-2">
          <component :is="item.icon" />
        </el-icon>
        <span>{{ item.label }}</span>
      </el-menu-item>
    </template>
  </el-menu>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import type { AdminMenuItem, AdminMenuMode } from '@/types/menu'
defineProps<{ items: AdminMenuItem[]; mode?: AdminMenuMode }>()
const route = useRoute()
const activeIndex = computed(() => route.path)

</script>

<style scoped>
</style>


