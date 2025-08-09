<script setup lang="ts">
import { ElConfigProvider } from 'element-plus'
import { useLocaleStore } from '@/stores/locale'

const localeStore = useLocaleStore()
</script>

<template>
  <ElConfigProvider :locale="localeStore.elLocale()" :button="{ autoInsertSpace: true }">
    <RouterView />
  </ElConfigProvider>
</template>

<style>
</style>
