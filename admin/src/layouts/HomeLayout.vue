<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header class="bg-white shadow-md">
    <div class="container mx-auto px-4">
      <nav class="flex items-center justify-between h-16">
        <div class="flex items-center space-x-4">
           <RouterLink to="/" class="text-gray-600 hover:text-gray-900">Home</RouterLink>
           <RouterLink to="/products" class="text-gray-600 hover:text-gray-900">Products</RouterLink>
        </div>
        <RouterLink to="/login" class="px-4 py-2 text-sm text-white bg-gray-800 rounded hover:bg-gray-900">Admin</RouterLink>
      </nav>
    </div>
  </header>

  <main>
    <RouterView />
  </main>
</template>

<style>
</style>
